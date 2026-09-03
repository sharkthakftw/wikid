use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use std::fs;
use std::path::PathBuf;

pub use crate::feed::categories::POPULAR_CATEGORIES;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FeedProfile {
    pub version: u32,
    #[serde(default)]
    pub has_onboarded: bool,
    #[serde(default)]
    pub selected_categories: Vec<String>,
    pub total_likes: u32,
    pub total_seen: u32,
    pub category_scores: HashMap<String, i32>,
    pub liked_articles: HashSet<String>,
    pub seen_articles: HashSet<String>,
}

impl Default for FeedProfile {
    fn default() -> Self {
        let mut category_scores = HashMap::new();
        category_scores.insert("given names".to_string(), -1000);
        category_scores.insert("surnames".to_string(), -1000);
        category_scores.insert("disambiguation pages".to_string(), -1000);

        Self {
            version: 1,
            has_onboarded: false,
            selected_categories: Vec::new(),
            total_likes: 0,
            total_seen: 0,
            category_scores,
            liked_articles: HashSet::new(),
            seen_articles: HashSet::new(),
        }
    }
}

impl FeedProfile {
    pub fn profile_path() -> PathBuf {
        crate::paths::config_dir().join("feed_profile.json")
    }

    pub fn load() -> Self {
        let path = Self::profile_path();
        let loaded = fs::read_to_string(&path)
            .ok()
            .and_then(|content| serde_json::from_str::<FeedProfile>(&content).ok());

        if let Some(mut profile) = loaded {
            let mut normalized = HashMap::with_capacity(profile.category_scores.len());
            for (k, v) in profile.category_scores {
                normalized.insert(k.to_lowercase(), v.max(0));
            }
            profile.category_scores = normalized;
            return profile;
        }
        let profile = Self::default();
        profile.save();
        profile
    }

    pub fn save(&self) {
        let path = Self::profile_path();
        if let Some(parent) = path.parent() {
            let _ = fs::create_dir_all(parent);
        }
        if let Ok(pretty_json) = serde_json::to_string_pretty(self) {
            let _ = fs::write(path, pretty_json);
        }
    }

    pub fn reset(&mut self) {
        *self = Self::default();
        self.save();
    }

    pub fn record_engagement(&mut self, categories: &[String], points: i32) {
        for cat in categories {
            let cat_lower = cat.to_lowercase();
            let current = self.category_scores.entry(cat_lower).or_insert(0);
            *current = (*current + points).max(0);
        }
    }

    pub fn mark_liked(&mut self, title: &str, categories: &[String]) -> bool {
        let is_liked = if self.liked_articles.contains(title) {
            self.liked_articles.remove(title);
            self.record_engagement(categories, -50);
            if self.total_likes > 0 {
                self.total_likes -= 1;
            }
            false
        } else {
            self.liked_articles.insert(title.to_string());
            self.record_engagement(categories, 50);
            self.total_likes += 1;
            true
        };
        self.save();
        is_liked
    }

    pub fn mark_seen(&mut self, title: &str) {
        if !self.seen_articles.contains(title) {
            self.seen_articles.insert(title.to_string());
            self.total_seen += 1;
            self.save();
        }
    }

    pub fn score_for_categories(&self, categories: &[String]) -> i32 {
        let mut score = 0;
        let mut buf = [0u8; 64];
        for cat in categories {
            let cat_score = if cat.len() <= buf.len() && cat.is_ascii() {
                let bytes = cat.as_bytes();
                for (i, &b) in bytes.iter().enumerate() {
                    buf[i] = b.to_ascii_lowercase();
                }
                let lower_str = std::str::from_utf8(&buf[..bytes.len()]).unwrap_or("");
                self.category_scores.get(lower_str).copied()
            } else {
                self.category_scores
                    .iter()
                    .find(|(k, _)| k.eq_ignore_ascii_case(cat))
                    .map(|(_, &s)| s)
            };
            if let Some(s) = cat_score {
                score += s;
            }
        }
        score
    }

    pub fn complete_onboarding(&mut self, chosen_indices: &[usize]) {
        self.selected_categories.clear();
        for &idx in chosen_indices {
            if let Some((_, label, subcats)) = POPULAR_CATEGORIES.get(idx) {
                self.selected_categories.push(label.to_string());
                for subcat in *subcats {
                    let current = self.category_scores.entry(subcat.to_lowercase()).or_insert(0);
                    *current += 200;
                }
            }
        }
        self.has_onboarded = true;
        self.save();
    }

    pub fn get_active_subcategories(&self) -> Vec<String> {
        let mut subcats = Vec::new();
        for (display_name, label, subcat_list) in POPULAR_CATEGORIES {
            if self
                .selected_categories
                .iter()
                .any(|c| c == *label || c == *display_name)
            {
                for s in *subcat_list {
                    subcats.push(s.to_string());
                }
            }
        }
        if subcats.is_empty() {
            for (_, _, subcat_list) in POPULAR_CATEGORIES {
                for s in *subcat_list {
                    subcats.push(s.to_string());
                }
            }
        }
        subcats
    }
}
