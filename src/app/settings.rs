use crate::app::App;
use crate::config::Config;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SettingItem {
    LikedReadonly,
    AutoRestoreSession,
    ConfirmQuit,
    HintMode,
    RoundedBorders,
    Icons,
    ScrollIndicator,
    Stats,
    DimInactivePanes,
    HeadingMarker,
    ScrollLines,
    UnderlineLinks,
    ShowFootnotes,
    ShowExternalLinks,
    TocSectionNumbers,
    CodeLineNumbers,
    ShowImages,
    ImageProtocol,
    HalfblockFilter,
    SearchLimit,
    NetworkTimeout,
    OfflineCache,
    CacheLifetime,
    MouseSupport,
    ScrollSpeed,
}

impl SettingItem {
    pub const ALL: &'static [SettingItem] = &[
        SettingItem::LikedReadonly,
        SettingItem::AutoRestoreSession,
        SettingItem::ConfirmQuit,
        SettingItem::HintMode,
        SettingItem::RoundedBorders,
        SettingItem::Icons,
        SettingItem::ScrollIndicator,
        SettingItem::Stats,
        SettingItem::DimInactivePanes,
        SettingItem::HeadingMarker,
        SettingItem::ScrollLines,
        SettingItem::UnderlineLinks,
        SettingItem::ShowFootnotes,
        SettingItem::ShowExternalLinks,
        SettingItem::TocSectionNumbers,
        SettingItem::CodeLineNumbers,
        SettingItem::ShowImages,
        SettingItem::ImageProtocol,
        SettingItem::HalfblockFilter,
        SettingItem::SearchLimit,
        SettingItem::NetworkTimeout,
        SettingItem::OfflineCache,
        SettingItem::CacheLifetime,
        SettingItem::MouseSupport,
        SettingItem::ScrollSpeed,
    ];

    pub fn section(&self) -> &'static str {
        match self {
            SettingItem::LikedReadonly
            | SettingItem::AutoRestoreSession
            | SettingItem::ConfirmQuit
            | SettingItem::HintMode => "general",
            SettingItem::RoundedBorders
            | SettingItem::Icons
            | SettingItem::ScrollIndicator
            | SettingItem::Stats
            | SettingItem::DimInactivePanes => "ui",
            SettingItem::HeadingMarker
            | SettingItem::ScrollLines
            | SettingItem::UnderlineLinks
            | SettingItem::ShowFootnotes
            | SettingItem::ShowExternalLinks
            | SettingItem::TocSectionNumbers
            | SettingItem::CodeLineNumbers
            | SettingItem::ShowImages
            | SettingItem::ImageProtocol
            | SettingItem::HalfblockFilter => "reader",
            SettingItem::SearchLimit => "search",
            SettingItem::NetworkTimeout
            | SettingItem::OfflineCache
            | SettingItem::CacheLifetime => "network",
            SettingItem::MouseSupport | SettingItem::ScrollSpeed => "input",
        }
    }

    pub fn label(&self) -> &'static str {
        match self {
            SettingItem::LikedReadonly => "liked list read-only",
            SettingItem::AutoRestoreSession => "auto-restore last session",
            SettingItem::ConfirmQuit => "confirm before quitting",
            SettingItem::HintMode => "continue reading hints",
            SettingItem::RoundedBorders => "rounded borders",
            SettingItem::Icons => "icons",
            SettingItem::ScrollIndicator => "scroll indicator",
            SettingItem::Stats => "wikipedia live stats",
            SettingItem::DimInactivePanes => "dim inactive split panes",
            SettingItem::HeadingMarker => "heading marker",
            SettingItem::ScrollLines => "scroll lines per step",
            SettingItem::UnderlineLinks => "underline links",
            SettingItem::ShowFootnotes => "show footnotes & citations",
            SettingItem::ShowExternalLinks => "show external links section",
            SettingItem::TocSectionNumbers => "toc section numbers",
            SettingItem::CodeLineNumbers => "code line numbers",
            SettingItem::ShowImages => "render images",
            SettingItem::ImageProtocol => "graphics protocol",
            SettingItem::HalfblockFilter => "halfblock filter",
            SettingItem::SearchLimit => "search results limit",
            SettingItem::NetworkTimeout => "request timeout",
            SettingItem::OfflineCache => "offline article cache",
            SettingItem::CacheLifetime => "cache lifetime",
            SettingItem::MouseSupport => "mouse support",
            SettingItem::ScrollSpeed => "mouse scroll speed",
        }
    }

    pub fn description(&self) -> &'static str {
        match self {
            SettingItem::LikedReadonly => "prevent manual deletion of articles from liked list",
            SettingItem::AutoRestoreSession => "automatically restore last session on startup",
            SettingItem::ConfirmQuit => "prompt for confirmation when exiting wikid",
            SettingItem::HintMode => {
                "hint style for continue reading list (semantic, numbered, none)"
            }
            SettingItem::RoundedBorders => "use rounded border corners instead of sharp",
            SettingItem::Icons => "display nerd fonts",
            SettingItem::ScrollIndicator => {
                "display scrollbar track on right edge of content panes"
            }
            SettingItem::Stats => "display live wikipedia statistics on launch screen",
            SettingItem::DimInactivePanes => {
                "subtly dim unfocused panes in multi-pane splits"
            }
            SettingItem::HeadingMarker => "display colored bar marker (▍) before section headings",
            SettingItem::ScrollLines => "number of lines to scroll per j/k press (1-20)",
            SettingItem::UnderlineLinks => "display underlined modifier on article links",
            SettingItem::ShowFootnotes => "show inline reference numbers and references section",
            SettingItem::ShowExternalLinks => "show the external links section at the bottom",
            SettingItem::TocSectionNumbers => "display hierarchical numbers in table of contents",
            SettingItem::CodeLineNumbers => "display line numbers in code blocks",
            SettingItem::ShowImages => "render inline article images and diagrams",
            SettingItem::ImageProtocol => {
                "graphics rendering protocol (auto, kitty, halfblocks, off)"
            }
            SettingItem::HalfblockFilter => {
                "halfblock image resampling filter (nearest, triangle, catmullrom, gaussian, lanczos3)"
            }
            SettingItem::SearchLimit => "maximum number of search results to fetch (5-50)",
            SettingItem::NetworkTimeout => "network request timeout in seconds (2-60s)",
            SettingItem::OfflineCache => {
                "cache downloaded articles in ~/.cache/wikid for offline reading"
            }
            SettingItem::CacheLifetime => "hours before cached articles are re-downloaded (1-168h)",
            SettingItem::MouseSupport => "enable mouse clicks, tab switching, and scroll wheel",
            SettingItem::ScrollSpeed => "number of lines to scroll per mouse wheel tick (1-20)",
        }
    }
}

impl App {
    pub fn adjust_selected_setting(&mut self, delta: i32) {
        if let Some(item) = SettingItem::ALL
            .get(self.settings_modal.cursor_idx)
            .copied()
        {
            match item {
                SettingItem::ScrollLines => {
                    let cur = self.config.reader.scroll_lines as i32;
                    let new_val = if delta == 0 {
                        if cur >= 20 {
                            1
                        } else {
                            cur + 1
                        }
                    } else {
                        (cur + delta).clamp(1, 20)
                    };
                    self.config.reader.scroll_lines = new_val as usize;
                }
                SettingItem::LikedReadonly => {
                    self.config.general.liked_readonly = !self.config.general.liked_readonly;
                }
                SettingItem::AutoRestoreSession => {
                    self.config.general.auto_restore_session =
                        !self.config.general.auto_restore_session;
                }
                SettingItem::ConfirmQuit => {
                    self.config.general.confirm_quit = !self.config.general.confirm_quit;
                }
                SettingItem::HintMode => {
                    self.config.general.hint_mode = match self.config.general.hint_mode {
                        crate::config::HintMode::Semantic => {
                            if delta < 0 {
                                crate::config::HintMode::None
                            } else {
                                crate::config::HintMode::Numbered
                            }
                        }
                        crate::config::HintMode::Numbered => {
                            if delta < 0 {
                                crate::config::HintMode::Semantic
                            } else {
                                crate::config::HintMode::None
                            }
                        }
                        crate::config::HintMode::None => {
                            if delta < 0 {
                                crate::config::HintMode::Numbered
                            } else {
                                crate::config::HintMode::Semantic
                            }
                        }
                    };
                }
                SettingItem::RoundedBorders => {
                    self.config.ui.rounded_borders = !self.config.ui.rounded_borders;
                }
                SettingItem::Icons => {
                    self.config.ui.icons = !self.config.ui.icons;
                }
                SettingItem::ScrollIndicator => {
                    self.config.ui.scroll_indicator = !self.config.ui.scroll_indicator;
                }
                SettingItem::Stats => {
                    self.config.ui.stats = !self.config.ui.stats;
                }
                SettingItem::DimInactivePanes => {
                    self.config.ui.dim_inactive_panes = !self.config.ui.dim_inactive_panes;
                }
                SettingItem::HeadingMarker => {
                    self.config.reader.heading_marker = !self.config.reader.heading_marker;
                }
                SettingItem::UnderlineLinks => {
                    self.config.reader.underline_links = !self.config.reader.underline_links;
                }
                SettingItem::ShowFootnotes => {
                    self.config.reader.show_footnotes = !self.config.reader.show_footnotes;
                }
                SettingItem::ShowExternalLinks => {
                    self.config.reader.show_external_links =
                        !self.config.reader.show_external_links;
                }
                SettingItem::TocSectionNumbers => {
                    self.config.reader.toc_section_numbers =
                        !self.config.reader.toc_section_numbers;
                }
                SettingItem::CodeLineNumbers => {
                    self.config.reader.code_line_numbers = !self.config.reader.code_line_numbers;
                }
                SettingItem::ShowImages => {
                    self.config.reader.show_images = !self.config.reader.show_images;
                }
                SettingItem::ImageProtocol => {
                    self.config.reader.image_protocol = match self.config.reader.image_protocol {
                        crate::config::ImageProtocol::Auto => {
                            if delta < 0 {
                                crate::config::ImageProtocol::Off
                            } else {
                                crate::config::ImageProtocol::Kitty
                            }
                        }
                        crate::config::ImageProtocol::Kitty => {
                            if delta < 0 {
                                crate::config::ImageProtocol::Auto
                            } else {
                                crate::config::ImageProtocol::Halfblocks
                            }
                        }
                        crate::config::ImageProtocol::Halfblocks => {
                            if delta < 0 {
                                crate::config::ImageProtocol::Kitty
                            } else {
                                crate::config::ImageProtocol::Off
                            }
                        }
                        crate::config::ImageProtocol::Off => {
                            if delta < 0 {
                                crate::config::ImageProtocol::Halfblocks
                            } else {
                                crate::config::ImageProtocol::Auto
                            }
                        }
                    };
                }
                SettingItem::HalfblockFilter => {
                    self.config.reader.halfblock_filter = match self.config.reader.halfblock_filter
                    {
                        crate::config::HalfblockFilter::Nearest => {
                            if delta < 0 {
                                crate::config::HalfblockFilter::Lanczos3
                            } else {
                                crate::config::HalfblockFilter::Triangle
                            }
                        }
                        crate::config::HalfblockFilter::Triangle => {
                            if delta < 0 {
                                crate::config::HalfblockFilter::Nearest
                            } else {
                                crate::config::HalfblockFilter::Catmullrom
                            }
                        }
                        crate::config::HalfblockFilter::Catmullrom => {
                            if delta < 0 {
                                crate::config::HalfblockFilter::Triangle
                            } else {
                                crate::config::HalfblockFilter::Gaussian
                            }
                        }
                        crate::config::HalfblockFilter::Gaussian => {
                            if delta < 0 {
                                crate::config::HalfblockFilter::Catmullrom
                            } else {
                                crate::config::HalfblockFilter::Lanczos3
                            }
                        }
                        crate::config::HalfblockFilter::Lanczos3 => {
                            if delta < 0 {
                                crate::config::HalfblockFilter::Gaussian
                            } else {
                                crate::config::HalfblockFilter::Nearest
                            }
                        }
                    };
                    for tab in &mut self.tabs {
                        for pane in &mut tab.panes {
                            pane.halfblock_cache.clear();
                        }
                    }
                }
                SettingItem::SearchLimit => {
                    let cur = self.config.search.limit as i32;
                    let step = 5;
                    let new_val = if delta == 0 {
                        if cur >= 50 {
                            5
                        } else {
                            cur + step
                        }
                    } else {
                        (cur + delta * step).clamp(5, 50)
                    };
                    self.config.search.limit = new_val as usize;
                }
                SettingItem::NetworkTimeout => {
                    let cur = self.config.network.timeout as i32;
                    let step = 2;
                    let new_val = if delta == 0 {
                        if cur >= 60 {
                            2
                        } else {
                            cur + step
                        }
                    } else {
                        (cur + delta * step).clamp(2, 60)
                    };
                    self.config.network.timeout = new_val as u64;
                }
                SettingItem::OfflineCache => {
                    self.config.network.offline_cache = !self.config.network.offline_cache;
                }
                SettingItem::CacheLifetime => {
                    let cur = self.config.network.cache_lifetime as i32;
                    let step = 6;
                    let new_val = if delta == 0 {
                        if cur >= 168 {
                            1
                        } else {
                            cur + step
                        }
                    } else {
                        (cur + delta * step).clamp(1, 168)
                    };
                    self.config.network.cache_lifetime = new_val as u64;
                }
                SettingItem::MouseSupport => {
                    self.config.input.mouse_support = !self.config.input.mouse_support;
                }
                SettingItem::ScrollSpeed => {
                    let cur = self.config.input.scroll_speed as i32;
                    let new_val = if delta == 0 {
                        if cur >= 20 {
                            1
                        } else {
                            cur + 1
                        }
                    } else {
                        (cur + delta).clamp(1, 20)
                    };
                    self.config.input.scroll_speed = new_val as usize;
                }
            }
            self.config.save();
            self.config.update_mtime();
        }
    }

    pub fn reset_selected_setting(&mut self) {
        if let Some(item) = SettingItem::ALL
            .get(self.settings_modal.cursor_idx)
            .copied()
        {
            let default_config = Config::default();
            match item {
                SettingItem::LikedReadonly => {
                    self.config.general.liked_readonly = default_config.general.liked_readonly;
                }
                SettingItem::AutoRestoreSession => {
                    self.config.general.auto_restore_session =
                        default_config.general.auto_restore_session;
                }
                SettingItem::ConfirmQuit => {
                    self.config.general.confirm_quit = default_config.general.confirm_quit;
                }
                SettingItem::HintMode => {
                    self.config.general.hint_mode = default_config.general.hint_mode;
                }
                SettingItem::RoundedBorders => {
                    self.config.ui.rounded_borders = default_config.ui.rounded_borders;
                }
                SettingItem::Icons => {
                    self.config.ui.icons = default_config.ui.icons;
                }
                SettingItem::ScrollIndicator => {
                    self.config.ui.scroll_indicator = default_config.ui.scroll_indicator;
                }
                SettingItem::Stats => {
                    self.config.ui.stats = default_config.ui.stats;
                }
                SettingItem::DimInactivePanes => {
                    self.config.ui.dim_inactive_panes = default_config.ui.dim_inactive_panes;
                }
                SettingItem::HeadingMarker => {
                    self.config.reader.heading_marker = default_config.reader.heading_marker;
                }
                SettingItem::ScrollLines => {
                    self.config.reader.scroll_lines = default_config.reader.scroll_lines;
                }
                SettingItem::UnderlineLinks => {
                    self.config.reader.underline_links = default_config.reader.underline_links;
                }
                SettingItem::ShowFootnotes => {
                    self.config.reader.show_footnotes = default_config.reader.show_footnotes;
                }
                SettingItem::ShowExternalLinks => {
                    self.config.reader.show_external_links =
                        default_config.reader.show_external_links;
                }
                SettingItem::TocSectionNumbers => {
                    self.config.reader.toc_section_numbers =
                        default_config.reader.toc_section_numbers;
                }
                SettingItem::CodeLineNumbers => {
                    self.config.reader.code_line_numbers = default_config.reader.code_line_numbers;
                }
                SettingItem::ShowImages => {
                    self.config.reader.show_images = default_config.reader.show_images;
                }
                SettingItem::ImageProtocol => {
                    self.config.reader.image_protocol = default_config.reader.image_protocol;
                }
                SettingItem::HalfblockFilter => {
                    self.config.reader.halfblock_filter = default_config.reader.halfblock_filter;
                    for tab in &mut self.tabs {
                        for pane in &mut tab.panes {
                            pane.halfblock_cache.clear();
                        }
                    }
                }
                SettingItem::SearchLimit => {
                    self.config.search.limit = default_config.search.limit;
                }
                SettingItem::NetworkTimeout => {
                    self.config.network.timeout = default_config.network.timeout;
                }
                SettingItem::OfflineCache => {
                    self.config.network.offline_cache = default_config.network.offline_cache;
                }
                SettingItem::CacheLifetime => {
                    self.config.network.cache_lifetime = default_config.network.cache_lifetime;
                }
                SettingItem::MouseSupport => {
                    self.config.input.mouse_support = default_config.input.mouse_support;
                }
                SettingItem::ScrollSpeed => {
                    self.config.input.scroll_speed = default_config.input.scroll_speed;
                }
            }
            self.config.save();
            self.config.update_mtime();
        }
    }

    pub fn reset_all_settings(&mut self) {
        self.config.current = Config::default();
        self.config.save();
        self.config.update_mtime();
    }
}
