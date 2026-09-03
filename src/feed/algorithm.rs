use crate::feed::profile::FeedProfile;
use std::sync::atomic::{AtomicU64, Ordering};

static RNG_STATE: AtomicU64 = AtomicU64::new(0);

#[derive(Debug, Clone)]
pub struct FeedItem {
    pub title: String,
    pub short_description: Option<String>,
    pub snippet: String,
    pub categories: Vec<String>,
    pub is_liked: bool,
}

pub enum SelectionStrategy {
    WeightedCategory,
    TopCategory,
    RandomExploration,
}

pub fn rand_u64() -> u64 {
    let prev = RNG_STATE.fetch_add(0x9e3779b97f4a7c15, Ordering::Relaxed);
    let mut z = if prev == 0 {
        let nanos = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map(|d| d.as_nanos())
            .unwrap_or(1234567) as u64;
        let seed = (nanos ^ 0x517cc1b727220a95).wrapping_add(0x9e3779b97f4a7c15);
        RNG_STATE.store(seed.wrapping_add(0x9e3779b97f4a7c15), Ordering::Relaxed);
        seed
    } else {
        prev
    };
    z = (z ^ (z >> 30)).wrapping_mul(0xbf58476d1ce4e5b9);
    z = (z ^ (z >> 27)).wrapping_mul(0x94d049bb133111eb);
    z ^ (z >> 31)
}

pub fn shuffle<T>(slice: &mut [T]) {
    for i in (1..slice.len()).rev() {
        let j = (rand_u64() as usize) % (i + 1);
        slice.swap(i, j);
    }
}

pub fn choose_strategy() -> SelectionStrategy {
    let roll = (rand_u64() % 100) as u8;
    if roll < 40 {
        SelectionStrategy::WeightedCategory
    } else if roll < 82 {
        SelectionStrategy::TopCategory
    } else {
        SelectionStrategy::RandomExploration
    }
}

pub fn select_best_item(candidates: &mut Vec<FeedItem>, profile: &FeedProfile) -> Option<FeedItem> {
    if candidates.is_empty() {
        return None;
    }

    let strategy = choose_strategy();
    match strategy {
        SelectionStrategy::RandomExploration => {
            let idx = (rand_u64() as usize) % candidates.len();
            Some(candidates.swap_remove(idx))
        }
        SelectionStrategy::TopCategory => {
            let mut best_idx = 0;
            let mut best_score = i32::MIN;

            for (idx, item) in candidates.iter().enumerate() {
                let score = profile.score_for_categories(&item.categories);
                if score > best_score {
                    best_score = score;
                    best_idx = idx;
                }
            }

            Some(candidates.swap_remove(best_idx))
        }
        SelectionStrategy::WeightedCategory => {
            let scores: Vec<u64> = candidates
                .iter()
                .map(|item| (profile.score_for_categories(&item.categories).max(0) as u64) + 1)
                .collect();
            let total_weight: u64 = scores.iter().sum();
            if total_weight == 0 {
                let idx = (rand_u64() as usize) % candidates.len();
                return Some(candidates.swap_remove(idx));
            }
            let mut roll = rand_u64() % total_weight;
            let mut chosen_idx = 0;
            for (idx, &w) in scores.iter().enumerate() {
                if roll < w {
                    chosen_idx = idx;
                    break;
                }
                roll -= w;
            }
            Some(candidates.swap_remove(chosen_idx))
        }
    }
}

pub fn rank_batch(
    mut candidates: Vec<FeedItem>,
    profile: &FeedProfile,
    read_articles: &std::collections::HashSet<String>,
) -> Vec<FeedItem> {
    candidates.retain(|item| {
        let title_lower = item.title.to_lowercase();
        !title_lower.starts_with("portal:")
            && !title_lower.starts_with("category:")
            && !profile.seen_articles.contains(&item.title)
            && !read_articles.contains(&title_lower)
    });
    let mut ranked = Vec::with_capacity(candidates.len());
    while !candidates.is_empty() {
        if let Some(item) = select_best_item(&mut candidates, profile) {
            ranked.push(item);
        } else {
            break;
        }
    }
    ranked
}
