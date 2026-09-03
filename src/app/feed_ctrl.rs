use crate::app::{App, InputMode};

impl App {
    pub fn open_daily_feed_modal(&mut self, kind: crate::ui::modals::DailyFeedKind) {
        if self.daily_feed.is_none() {
            self.send_fetch_daily_feed();
        }
        self.daily_feed_modal = Some(crate::ui::modals::DailyFeedModalState {
            kind,
            cursor_idx: 0,
            link_idx: 0,
            otd_tab: crate::ui::modals::OnThisDayTab::Events,
            cache: std::cell::RefCell::new(crate::ui::modals::DailyFeedCache::default()),
        });
        self.input_mode = InputMode::DailyFeedModal;
    }

    pub fn close_daily_feed_modal(&mut self) {
        self.daily_feed_modal = None;
        self.input_mode = InputMode::Normal;
    }

    pub fn maybe_fetch_feed_batch(&mut self) {
        if !self.feed.is_fetching && self.feed.active_idx + 3 >= self.feed.items.len() {
            self.feed.is_fetching = true;
            self.send_fetch_feed_batch();
        }
    }

    pub fn toggle_feed_mode(&mut self) {
        let is_active = self.feed.toggle_active();
        if is_active {
            if !self.feed.profile.has_onboarded {
                self.input_mode = InputMode::CategoryOnboarding;
            } else if self.feed.items.is_empty() {
                self.maybe_fetch_feed_batch();
            }
        }
    }

    pub fn submit_category_onboarding(&mut self) {
        let chosen_indices: Vec<usize> = self
            .onboarding
            .selected
            .iter()
            .enumerate()
            .filter_map(|(idx, &sel)| if sel { Some(idx) } else { None })
            .collect();

        self.feed.profile.complete_onboarding(&chosen_indices);
        self.input_mode = InputMode::Normal;
        if self.feed.items.is_empty() {
            self.maybe_fetch_feed_batch();
        }
    }

    pub fn reset_feed(&mut self) {
        self.feed.reset();
        self.saved_lists.clear_list("liked");
        self.onboarding.cursor_idx = 0;
        self.onboarding.selected = vec![
            false, false, false, false, true, false, false, true, true, false, false, true,
        ];
        self.input_mode = InputMode::CategoryOnboarding;
        self.set_status_message("feed reset: select initial categories");
    }

    pub fn toggle_feed_like(&mut self) {
        if let Some((title, _snippet, is_liked)) = self.feed.toggle_like() {
            self.saved_lists
                .set_article_in_list("liked", "Liked", &title, is_liked);
        }
    }
}
