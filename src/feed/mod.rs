pub mod algorithm;
pub mod categories;
pub mod profile;

use crate::feed::algorithm::FeedItem;
use crate::feed::profile::FeedProfile;

pub struct FeedState {
    pub active: bool,
    pub items: Vec<FeedItem>,
    pub active_idx: usize,
    pub profile: FeedProfile,
    pub is_fetching: bool,
}

impl Default for FeedState {
    fn default() -> Self {
        Self {
            active: false,
            items: Vec::new(),
            active_idx: 0,
            profile: FeedProfile::load(),
            is_fetching: false,
        }
    }
}

impl FeedState {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn toggle_active(&mut self) -> bool {
        self.active = !self.active;
        self.active
    }

    pub fn reset(&mut self) {
        self.profile.reset();
        self.items.clear();
        self.active_idx = 0;
        self.is_fetching = false;
    }

    pub fn add_item(&mut self, item: FeedItem) {
        self.items.push(item);
    }

    pub fn next_post(&mut self) {
        if self.items.is_empty() {
            return;
        }
        if self.active_idx + 1 < self.items.len() {
            let item = &self.items[self.active_idx];
            if !item.is_liked {
                self.profile.mark_seen(&item.title);
            }
            self.active_idx += 1;
        }
    }

    pub fn prev_post(&mut self) {
        if self.active_idx > 0 {
            self.active_idx -= 1;
        }
    }

    pub fn toggle_like(&mut self) -> Option<(String, Option<String>, bool)> {
        if self.items.is_empty() {
            return None;
        }
        let item = &mut self.items[self.active_idx];
        let is_liked = self.profile.mark_liked(&item.title, &item.categories);
        item.is_liked = is_liked;
        Some((item.title.clone(), item.short_description.clone(), is_liked))
    }

    pub fn current_item(&self) -> Option<&FeedItem> {
        self.items.get(self.active_idx)
    }
}
