use super::types::{SpanStyle, StyledChunk};
use crate::parser::utils::decode_html_entities;

pub fn parse_story_html(input: &str) -> (Vec<StyledChunk>, Vec<String>) {
    let mut chunks = Vec::new();
    let mut links = Vec::new();
    let mut current_text = String::new();
    let mut in_bold = false;
    let mut in_italic = false;
    let mut current_link: Option<(usize, String)> = None;

    let flush = |chunks: &mut Vec<StyledChunk>,
                 current_text: &mut String,
                 in_bold: bool,
                 in_italic: bool,
                 current_link: &Option<(usize, String)>| {
        if current_text.is_empty() {
            return;
        }
        let style = match (current_link, in_bold, in_italic) {
            (Some((l_idx, target)), true, _) => SpanStyle::BoldLink {
                link_idx: *l_idx,
                title: target.clone(),
            },
            (Some((l_idx, target)), false, _) => SpanStyle::Link {
                link_idx: *l_idx,
                title: target.clone(),
            },
            (None, true, _) => SpanStyle::Bold,
            (None, false, true) => SpanStyle::Italic,
            (None, false, false) => SpanStyle::Normal,
        };
        let decoded = decode_html_entities(current_text);
        chunks.push(StyledChunk {
            text: decoded.into_owned(),
            style,
        });
        current_text.clear();
    };

    let mut chars = input.chars();
    while let Some(c) = chars.next() {
        if c == '<' {
            if chars.as_str().starts_with("!--") {
                while let Some(nc) = chars.next() {
                    if nc == '-' && chars.as_str().starts_with("->") {
                        chars.next();
                        chars.next();
                        break;
                    }
                }
                continue;
            }

            let mut tag = String::new();
            for nc in chars.by_ref() {
                if nc == '>' {
                    break;
                }
                tag.push(nc);
            }

            let tag_lower = tag.to_lowercase();
            let tag_name = tag_lower.split_whitespace().next().unwrap_or("");

            match tag_name {
                "b" | "strong" => {
                    flush(
                        &mut chunks,
                        &mut current_text,
                        in_bold,
                        in_italic,
                        &current_link,
                    );
                    in_bold = true;
                }
                "/b" | "/strong" => {
                    flush(
                        &mut chunks,
                        &mut current_text,
                        in_bold,
                        in_italic,
                        &current_link,
                    );
                    in_bold = false;
                }
                "i" | "em" => {
                    flush(
                        &mut chunks,
                        &mut current_text,
                        in_bold,
                        in_italic,
                        &current_link,
                    );
                    in_italic = true;
                }
                "/i" | "/em" => {
                    flush(
                        &mut chunks,
                        &mut current_text,
                        in_bold,
                        in_italic,
                        &current_link,
                    );
                    in_italic = false;
                }
                "a" => {
                    flush(
                        &mut chunks,
                        &mut current_text,
                        in_bold,
                        in_italic,
                        &current_link,
                    );
                    let title = if let Some(pos) = tag.find("title=\"") {
                        let rest = &tag[pos + 7..];
                        rest.split('"').next().unwrap_or("").to_string()
                    } else if let Some(pos) = tag.find("href=\"./") {
                        let rest = &tag[pos + 8..];
                        rest.split('"').next().unwrap_or("").replace('_', " ")
                    } else {
                        String::new()
                    };
                    let l_idx = links.len();
                    links.push(title.clone());
                    current_link = Some((l_idx, title));
                }
                "/a" => {
                    flush(
                        &mut chunks,
                        &mut current_text,
                        in_bold,
                        in_italic,
                        &current_link,
                    );
                    current_link = None;
                }
                _ => {}
            }
            continue;
        }

        current_text.push(c);
    }

    flush(
        &mut chunks,
        &mut current_text,
        in_bold,
        in_italic,
        &current_link,
    );
    (chunks, links)
}

pub use crate::parser::utils::strip_html_tags;

pub fn wrap_story_spans(chunks: &[StyledChunk], max_width: usize) -> Vec<Vec<(String, SpanStyle)>> {
    let mut lines: Vec<Vec<(String, SpanStyle)>> = Vec::new();
    let mut current_line: Vec<(String, SpanStyle)> = Vec::new();
    let mut current_line_len = 0;
    let target_width = max_width.saturating_sub(4);

    for chunk in chunks {
        let mut word = String::new();
        for ch in chunk.text.chars() {
            if ch == ' ' {
                if !word.is_empty() {
                    let word_len = word.chars().count();
                    if current_line_len + word_len > target_width && current_line_len > 0 {
                        lines.push(current_line);
                        current_line = Vec::new();
                        current_line_len = 0;
                    }
                    current_line.push((word.clone(), chunk.style.clone()));
                    current_line_len += word_len;
                    word.clear();
                }
                if current_line_len > 0 && current_line_len < target_width {
                    current_line.push((" ".to_string(), chunk.style.clone()));
                    current_line_len += 1;
                }
            } else {
                word.push(ch);
            }
        }
        if !word.is_empty() {
            let word_len = word.chars().count();
            if current_line_len + word_len > target_width && current_line_len > 0 {
                lines.push(current_line);
                current_line = Vec::new();
                current_line_len = 0;
            }
            current_line.push((word, chunk.style.clone()));
            current_line_len += word_len;
        }
    }

    if !current_line.is_empty() {
        lines.push(current_line);
    }

    lines
}

fn normalize_search_str(s: &str) -> String {
    decode_html_entities(s)
        .replace(['\u{a0}', '\u{202f}'], " ")
        .replace(['–', '—', '−'], "-")
        .replace(['\u{2018}', '\u{2019}', '\u{201b}'], "'")
        .replace(['\u{201c}', '\u{201d}'], "\"")
}

fn char_eq_norm(a: char, b: char) -> bool {
    let a_norm = if a == '-' { ' ' } else { a };
    let b_norm = if b == '-' { ' ' } else { b };
    a_norm.eq_ignore_ascii_case(&b_norm) || a_norm.to_lowercase().eq(b_norm.to_lowercase())
}

fn find_case_insensitive_matches(text: &str, term: &str) -> Vec<(usize, usize)> {
    let mut matches = Vec::new();
    let term_chars: Vec<char> = term.chars().collect();
    if term_chars.is_empty() {
        return matches;
    }

    let text_chars: Vec<(usize, char)> = text.char_indices().collect();
    if text_chars.len() < term_chars.len() {
        return matches;
    }

    let allow_plural = term_chars.len() >= 4
        && term_chars
            .last()
            .is_some_and(|c| c.is_alphanumeric() && !c.eq_ignore_ascii_case(&'s'));

    let mut i = 0;
    while i + term_chars.len() <= text_chars.len() {
        let left_ok = i == 0 || !text_chars[i - 1].1.is_alphanumeric();
        if !left_ok {
            i += 1;
            continue;
        }

        let is_match = text_chars[i..i + term_chars.len()]
            .iter()
            .zip(&term_chars)
            .all(|((_, tc), mc)| char_eq_norm(*tc, *mc));

        if is_match {
            let mut end_idx = i + term_chars.len();
            if end_idx < text_chars.len() && text_chars[end_idx].1.is_alphanumeric() {
                if allow_plural
                    && text_chars[end_idx].1.eq_ignore_ascii_case(&'s')
                    && (end_idx + 1 == text_chars.len()
                        || !text_chars[end_idx + 1].1.is_alphanumeric())
                {
                    end_idx += 1;
                } else {
                    i += 1;
                    continue;
                }
            }

            let start_byte = text_chars[i].0;
            let end_byte = if end_idx < text_chars.len() {
                text_chars[end_idx].0
            } else {
                text.len()
            };
            matches.push((start_byte, end_byte));
            i = end_idx;
        } else {
            i += 1;
        }
    }
    matches
}

fn strip_prefix_icase<'a>(s: &'a str, prefix: &str) -> Option<&'a str> {
    if s.get(..prefix.len())
        .is_some_and(|p| p.eq_ignore_ascii_case(prefix))
    {
        Some(&s[prefix.len()..])
    } else {
        None
    }
}

fn strip_suffix_icase<'a>(s: &'a str, suffix: &str) -> Option<&'a str> {
    let split_at = s.len().checked_sub(suffix.len())?;
    if s.get(split_at..)
        .is_some_and(|p| p.eq_ignore_ascii_case(suffix))
    {
        Some(&s[..split_at])
    } else {
        None
    }
}

pub fn parse_onthisday_event(
    text: &str,
    pages: &[crate::api::daily_feed::PageSummary],
) -> (Vec<StyledChunk>, Vec<String>) {
    let mut match_targets: Vec<(String, String)> = Vec::new();

    for page in pages {
        let canonical = &page.title;
        let display = page.display_title();
        let norm_display = normalize_search_str(&display);
        match_targets.push((norm_display.clone(), canonical.clone()));

        let base_title = norm_display
            .split('(')
            .next()
            .unwrap_or(&norm_display)
            .trim();
        if base_title != norm_display && !base_title.is_empty() {
            match_targets.push((base_title.to_string(), canonical.clone()));
        }

        let mut bases = vec![base_title];
        if let Some((before_comma, _)) = base_title.split_once(',') {
            let trimmed = before_comma.trim();
            if trimmed.chars().count() >= 3 {
                match_targets.push((trimmed.to_string(), canonical.clone()));
                bases.push(trimmed);
            }
        }

        for b in bases {
            for art in ["the ", "a ", "an "] {
                if let Some(rest) = strip_prefix_icase(b, art)
                    && rest.chars().count() >= 3
                {
                    match_targets.push((rest.to_string(), canonical.clone()));
                }
            }

            for suffix in [
                " line",
                " battle",
                " war",
                " siege",
                " treaty",
                " expedition",
            ] {
                if let Some(stripped) = strip_suffix_icase(b, suffix)
                    && stripped.chars().count() >= 3
                {
                    match_targets.push((stripped.to_string(), canonical.clone()));
                }
            }
        }
    }

    match_targets.sort_by_key(|a| std::cmp::Reverse(a.0.chars().count()));
    let clean_text = normalize_search_str(text);

    let mut ranges: Vec<(usize, usize, String)> = Vec::new();
    for (term, canonical) in &match_targets {
        for (actual_start, actual_end) in find_case_insensitive_matches(&clean_text, term) {
            let overlaps = ranges.iter().any(|(s, e, _)| {
                (actual_start >= *s && actual_start < *e)
                    || (actual_end > *s && actual_end <= *e)
                    || (actual_start <= *s && actual_end >= *e)
            });

            if !overlaps {
                ranges.push((actual_start, actual_end, canonical.clone()));
            }
        }
    }

    ranges.sort_by_key(|r| r.0);

    let mut links = Vec::new();
    let mut chunks = Vec::new();
    let mut last_idx = 0;
    for (start, end, target) in ranges {
        if start > last_idx {
            chunks.push(StyledChunk {
                text: clean_text[last_idx..start].to_string(),
                style: SpanStyle::Normal,
            });
        }
        let l_idx = if let Some(pos) = links.iter().position(|t| t == &target) {
            pos
        } else {
            let idx = links.len();
            links.push(target.clone());
            idx
        };
        chunks.push(StyledChunk {
            text: clean_text[start..end].to_string(),
            style: SpanStyle::Link {
                link_idx: l_idx,
                title: target,
            },
        });
        last_idx = end;
    }
    if last_idx < clean_text.len() {
        chunks.push(StyledChunk {
            text: clean_text[last_idx..].to_string(),
            style: SpanStyle::Normal,
        });
    }

    if links.is_empty()
        && let Some(first_page) = pages.first()
    {
        links.push(first_page.title.clone());
        chunks = vec![StyledChunk {
            text: clean_text,
            style: SpanStyle::Link {
                link_idx: 0,
                title: first_page.title.clone(),
            },
        }];
    }

    (chunks, links)
}
