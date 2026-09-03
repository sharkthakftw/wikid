#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DailyFeedKind {
    News,
    OnThisDay,
    MostRead,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum OnThisDayTab {
    #[default]
    Events,
    Births,
    Deaths,
    Holidays,
}

#[derive(Clone, Debug, Default)]
pub struct ParsedStory {
    pub chunks: Vec<StyledChunk>,
    pub links: Vec<String>,
}

#[derive(Clone, Debug, Default)]
pub struct CachedWrappedItem {
    pub links: Vec<String>,
    pub wrapped_lines: Vec<Vec<(String, SpanStyle)>>,
    pub year_str: String,
    pub elapsed_str: String,
}

#[derive(Clone, Debug, Default)]
pub struct DailyFeedCache {
    pub news_parsed: Vec<ParsedStory>,
    pub news_width: usize,
    pub news_wrapped: Vec<CachedWrappedItem>,

    pub otd_events_parsed: Vec<ParsedStory>,
    pub otd_births_parsed: Vec<ParsedStory>,
    pub otd_deaths_parsed: Vec<ParsedStory>,
    pub otd_holidays_parsed: Vec<ParsedStory>,

    pub otd_cached_tab: Option<OnThisDayTab>,
    pub otd_cached_width: usize,
    pub otd_wrapped: Vec<CachedWrappedItem>,
}

#[derive(Clone, Debug)]
pub struct DailyFeedModalState {
    pub kind: DailyFeedKind,
    pub cursor_idx: usize,
    pub link_idx: usize,
    pub otd_tab: OnThisDayTab,
    pub cache: std::cell::RefCell<DailyFeedCache>,
}

impl Default for DailyFeedModalState {
    fn default() -> Self {
        Self {
            kind: DailyFeedKind::News,
            cursor_idx: 0,
            link_idx: 0,
            otd_tab: OnThisDayTab::Events,
            cache: std::cell::RefCell::new(DailyFeedCache::default()),
        }
    }
}

pub struct FeedEntry {
    pub title: String,
    pub target_article: String,
    pub suffix: Option<String>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum SpanStyle {
    Normal,
    Bold,
    Italic,
    Link { link_idx: usize, title: String },
    BoldLink { link_idx: usize, title: String },
}

#[derive(Debug, Clone)]
pub struct StyledChunk {
    pub text: String,
    pub style: SpanStyle,
}
