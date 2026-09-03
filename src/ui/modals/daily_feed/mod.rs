pub mod html;
pub mod most_read;
pub mod news;
pub mod on_this_day;
pub mod types;

pub use html::{parse_onthisday_event, parse_story_html, strip_html_tags, wrap_story_spans};
pub use news::{get_ongoing_links, get_recent_deaths_links, render_news_modal};
pub use on_this_day::{get_otd_tab_at, render_on_this_day_modal, today_date_str};
pub use types::{
    CachedWrappedItem, DailyFeedCache, DailyFeedKind, DailyFeedModalState, FeedEntry, OnThisDayTab,
    ParsedStory, SpanStyle, StyledChunk,
};

use crate::app::App;
use crate::theme;
use crate::ui::modals::utils::{centered_rect, render_modal_frame_at};
use ratatui::{layout::Rect, Frame};

pub fn compute_daily_feed_modal_area(container_rect: Rect, kind: DailyFeedKind) -> Rect {
    match kind {
        DailyFeedKind::MostRead => centered_rect(50, 57, container_rect),
        DailyFeedKind::OnThisDay => centered_rect(75, 69, container_rect),
        DailyFeedKind::News => centered_rect(75, 65, container_rect),
    }
}

pub fn get_feed_entries(app: &App, kind: DailyFeedKind) -> Vec<FeedEntry> {
    let feed = match &app.daily_feed {
        Some(f) => f,
        None => return Vec::new(),
    };

    match kind {
        DailyFeedKind::News => {
            let mut entries = Vec::new();
            for item in &feed.news {
                if let Some(first_link) = item.links.first() {
                    let summary = item.story.as_deref().unwrap_or("");
                    let clean_story = strip_html_tags(summary);
                    let target = first_link.title.clone();
                    entries.push(FeedEntry {
                        title: if !clean_story.is_empty() {
                            clean_story
                        } else {
                            first_link.display_title().to_string()
                        },
                        target_article: target,
                        suffix: None,
                    });
                }
            }
            if !feed.ongoing.is_empty() {
                let first_target = feed
                    .ongoing
                    .first()
                    .map(|o| o.target.clone())
                    .unwrap_or_default();
                entries.push(FeedEntry {
                    title: "Ongoing".to_string(),
                    target_article: first_target,
                    suffix: None,
                });
            }
            if !feed.recent_deaths.is_empty() {
                let first_target = feed
                    .recent_deaths
                    .first()
                    .map(|d| d.target.clone())
                    .unwrap_or_default();
                entries.push(FeedEntry {
                    title: "Recent deaths".to_string(),
                    target_article: first_target,
                    suffix: None,
                });
            }
            entries
        }
        DailyFeedKind::OnThisDay => {
            let otd_tab = app
                .daily_feed_modal
                .as_ref()
                .map(|m| m.otd_tab)
                .unwrap_or_default();
            let events_slice: &[crate::api::daily_feed::OnThisDayEvent] =
                if let Some(archive) = &feed.onthisday_all {
                    match otd_tab {
                        OnThisDayTab::Events => {
                            if !archive.events.is_empty() {
                                &archive.events
                            } else {
                                &feed.onthisday
                            }
                        }
                        OnThisDayTab::Births => &archive.births,
                        OnThisDayTab::Deaths => &archive.deaths,
                        OnThisDayTab::Holidays => &archive.holidays,
                    }
                } else {
                    &feed.onthisday
                };

            let mut entries = Vec::new();
            for event in events_slice {
                let target = event
                    .pages
                    .first()
                    .map(|p| p.title.clone())
                    .unwrap_or_default();
                if target.is_empty() {
                    continue;
                }
                let year_str = match event.year {
                    Some(y) if y < 0 => format!("{} BC", y.abs()),
                    Some(y) => format!("{}", y),
                    None => "Holiday".to_string(),
                };
                let clean_text = strip_html_tags(&event.text);
                let display = format!("[ {} ]  {}", year_str, clean_text);
                entries.push(FeedEntry {
                    title: display,
                    target_article: target,
                    suffix: None,
                });
            }
            entries
        }
        DailyFeedKind::MostRead => {
            let mut entries = Vec::new();
            if let Some(payload) = &feed.mostread {
                for (idx, item) in payload.articles.iter().take(25).enumerate() {
                    let rank_str = format!("{}. ", idx + 1);
                    let views_str = item.views.map(crate::api::stats::format_metric);
                    let display = format!("{}{}", rank_str, item.display_title());
                    entries.push(FeedEntry {
                        title: display,
                        target_article: item.title.clone(),
                        suffix: views_str.map(|v| format!("  {} views", v)),
                    });
                }
            }
            entries
        }
    }
}

pub fn render_daily_feed_modal(f: &mut Frame, app: &App, size: Rect) {
    let state = match &app.daily_feed_modal {
        Some(s) => s,
        None => return,
    };

    let (icon, title_text) = match state.kind {
        DailyFeedKind::News => (
            if app.config.ui.icons { "󰋫" } else { "" },
            "in the news".to_string(),
        ),
        DailyFeedKind::OnThisDay => (
            if app.config.ui.icons { "󰃭" } else { "" },
            format!("on this day · {}", today_date_str()),
        ),
        DailyFeedKind::MostRead => (
            if app.config.ui.icons { "󰄬" } else { "" },
            "most read".to_string(),
        ),
    };
    let accent_color = theme::BLUE;

    let modal_area = compute_daily_feed_modal_area(size, state.kind);
    let modal_block = render_modal_frame_at(
        f,
        modal_area,
        icon,
        &title_text,
        accent_color,
        app.config.ui.rounded_borders,
    );

    let entries = get_feed_entries(app, state.kind);
    let total = entries.len();
    let selected_idx = state.cursor_idx.min(total.saturating_sub(1));

    match state.kind {
        DailyFeedKind::News => {
            news::render_news_modal(
                f,
                app,
                modal_area,
                modal_block,
                selected_idx,
                state.link_idx,
            );
        }
        DailyFeedKind::OnThisDay => {
            on_this_day::render_on_this_day_modal(
                f,
                app,
                modal_area,
                modal_block,
                selected_idx,
                state.link_idx,
                state.otd_tab,
            );
        }
        DailyFeedKind::MostRead => {
            most_read::render_most_read_modal(
                f,
                app,
                &entries,
                modal_area,
                modal_block,
                selected_idx,
            );
        }
    }
}
