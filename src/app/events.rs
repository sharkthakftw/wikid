use crate::api::NetworkEvent;
use crate::app::pane::PaneContent;
use crate::app::App;
use crate::parser::parse_wikipedia_html;

impl App {
    pub fn handle_network_event(&mut self, ev: NetworkEvent) {
        match ev {
            NetworkEvent::SearchResult {
                request_id,
                pane_id,
                query,
                results,
            } => {
                if let Some(pane) = self.find_pane_mut(pane_id) {
                    if request_id >= pane.current_request_id {
                        pane.is_loading = false;
                        pane.loading_title = None;
                        pane.selected_idx = 0;
                        pane.scroll_offset = 0;
                        pane.toc_focused = false;
                        pane.content = PaneContent::SearchResults {
                            query,
                            items: results,
                        };
                    }
                }
            }
            NetworkEvent::ArticleResult {
                request_id,
                pane_id,
                title,
                content,
            } => {
                let is_current = self
                    .find_pane_mut(pane_id)
                    .is_some_and(|p| request_id >= p.current_request_id);

                if is_current {
                    self.record_recent_article(&title);
                    let width = self
                        .find_pane(pane_id)
                        .map(|p| {
                            if p.viewport_width > 0 {
                                p.viewport_width
                            } else {
                                80
                            }
                        })
                        .unwrap_or(80);
                    let render_opts = crate::app::pane::ArticleRenderOptions {
                        width,
                        show_footnotes: self.config.reader.show_footnotes,
                        show_external_links: self.config.reader.show_external_links,
                        heading_marker: self.config.reader.heading_marker,
                        code_line_numbers: self.config.reader.code_line_numbers,
                        show_icons: self.config.ui.icons,
                        show_images: self.config.reader.show_images,
                        max_image_height: self.config.reader.max_image_height,
                    };
                    if let Some(pane) = self.find_pane_mut(pane_id) {
                        pane.is_loading = false;
                        pane.loading_title = None;
                        pane.toc_focused = false;
                        let parsed_doc = parse_wikipedia_html(
                            &content,
                            render_opts.width,
                            render_opts.show_footnotes,
                            render_opts.show_external_links,
                            render_opts.heading_marker,
                            render_opts.code_line_numbers,
                            render_opts.show_icons,
                            render_opts.show_images,
                            render_opts.max_image_height,
                        );
                        pane.scroll_offset = pane
                            .scroll_offset
                            .min(parsed_doc.lines.len().saturating_sub(1));
                        let initial_link_idx = if !parsed_doc.links.is_empty() {
                            Some(0)
                        } else {
                            None
                        };
                        let image_urls: Vec<String> = parsed_doc
                            .images
                            .iter()
                            .map(|img| img.url.clone())
                            .collect();
                        pane.content = PaneContent::ArticleText {
                            title,
                            raw_html: content,
                            parsed_doc: Box::new(parsed_doc),
                            last_render_options: render_opts,
                        };
                        pane.selected_link_idx = initial_link_idx;
                        for img_url in image_urls {
                            self.send_fetch_image(img_url);
                        }
                    }
                }
            }
            NetworkEvent::ImageLoaded { url, path } => {
                for tab in &mut self.tabs {
                    for pane in &mut tab.panes {
                        pane.loaded_images.insert(url.clone(), path.clone());
                    }
                }
            }
            NetworkEvent::Error {
                request_id,
                pane_id,
                error,
            } => {
                if let Some(pane) = self.find_pane_mut(pane_id) {
                    if request_id >= pane.current_request_id {
                        pane.is_loading = false;
                        pane.loading_title = None;
                        pane.content = PaneContent::Error(error.to_string());
                    }
                }
            }
            NetworkEvent::FeedBatchLoaded { items } => {
                self.feed.is_fetching = false;
                let read_titles: std::collections::HashSet<String> = self
                    .recent_articles
                    .iter()
                    .map(|e| e.title.to_lowercase())
                    .collect();
                let ranked_items = crate::feed::algorithm::rank_batch(
                    items,
                    &self.feed.profile,
                    &read_titles,
                );
                for mut item in ranked_items {
                    item.is_liked = self.feed.profile.liked_articles.contains(&item.title)
                        || self.saved_lists.is_article_in_list("liked", &item.title);
                    self.feed.add_item(item);
                }
                if self.feed.items.is_empty() {
                    self.maybe_fetch_feed_batch();
                }
            }
            NetworkEvent::DailyFeedLoaded(feed) => {
                self.daily_feed = Some(*feed);
                if self.pending_open_tfa {
                    self.pending_open_tfa = false;
                    self.active_pane_mut().is_loading = false;
                    if let Some(tfa) = self.daily_feed.as_ref().and_then(|f| f.tfa.as_ref()) {
                        let title = tfa.display_title();
                        self.open_article(&title);
                    }
                }
            }
            NetworkEvent::StatsLoaded(stats) => {
                self.wiki_stats = stats;
            }
            NetworkEvent::UpdateCheckResult { latest_tag } => match latest_tag {
                Ok(tag) => {
                    let remote_ver = parse_semver_parts(&tag);
                    let current_ver = parse_semver_parts(env!("CARGO_PKG_VERSION"));
                    if remote_ver > current_ver {
                        self.set_status_message(format!(
                            "update available: {} (current: v{}) · run cargo install wikid",
                            tag,
                            env!("CARGO_PKG_VERSION")
                        ));
                    } else {
                        self.set_status_message(format!(
                            "wikid is up to date (v{})",
                            env!("CARGO_PKG_VERSION")
                        ));
                    }
                }
                Err(err) => {
                    self.set_status_message(format!("update check failed: {}", err));
                }
            },
            NetworkEvent::CategoryMembersLoaded { category, members } => {
                self.categories_modal.fetching_categories.remove(&category);
                self.categories_modal
                    .cached_members
                    .insert(category, members);
            }
        }
    }

    pub fn fetch_category_members_if_needed(&mut self, category: &str) {
        if !self.categories_modal.cached_members.contains_key(category)
            && !self.categories_modal.fetching_categories.contains(category)
        {
            self.categories_modal
                .fetching_categories
                .insert(category.to_string());
            self.network
                .send(crate::api::NetworkCommand::FetchCategoryMembers {
                    category: category.to_string(),
                    limit: 50,
                    timeout: self.config.network.timeout,
                });
        }
    }
}

fn parse_semver_parts(v: &str) -> (u32, u32, u32) {
    let clean = v.trim_start_matches('v').trim_start_matches('V').trim();
    let mut parts = clean.split('.').filter_map(|p| {
        p.chars()
            .take_while(|c| c.is_ascii_digit())
            .collect::<String>()
            .parse::<u32>()
            .ok()
    });
    (
        parts.next().unwrap_or(0),
        parts.next().unwrap_or(0),
        parts.next().unwrap_or(0),
    )
}
