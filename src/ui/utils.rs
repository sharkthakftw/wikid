use unicode_width::{UnicodeWidthChar, UnicodeWidthStr};

pub fn truncate_to_width(s: &str, max_width: usize) -> String {
    truncate_with_ellipsis(s, max_width, "...")
}

fn take_width_bytes(s: &str, max_width: usize) -> usize {
    let mut cur_w = 0;
    let mut byte_end = 0;
    for (idx, ch) in s.char_indices() {
        let ch_w = UnicodeWidthChar::width(ch).unwrap_or(0);
        if cur_w + ch_w > max_width {
            break;
        }
        cur_w += ch_w;
        byte_end = idx + ch.len_utf8();
    }
    byte_end
}

pub fn truncate_with_ellipsis(s: &str, max_width: usize, ellipsis: &str) -> String {
    let total_width = UnicodeWidthStr::width(s);
    if total_width <= max_width {
        return s.to_string();
    }
    if max_width == 0 {
        return String::new();
    }

    let ell_w = UnicodeWidthStr::width(ellipsis);
    if max_width <= ell_w {
        let byte_end = take_width_bytes(s, max_width);
        return s[..byte_end].to_string();
    }

    let byte_end = take_width_bytes(s, max_width - ell_w);
    format!("{}{}", &s[..byte_end], ellipsis)
}

pub fn wrap_text(text: &str, max_width: usize) -> Vec<String> {
    let mut lines = Vec::new();
    let mut current_line = String::new();
    let mut current_len = 0;

    for word in text.split_whitespace() {
        let word_len = UnicodeWidthStr::width(word);
        if current_line.is_empty() {
            current_line.push_str(word);
            current_len = word_len;
        } else if current_len + 1 + word_len <= max_width {
            current_line.push(' ');
            current_line.push_str(word);
            current_len += 1 + word_len;
        } else {
            lines.push(current_line);
            current_line = word.to_string();
            current_len = word_len;
        }
    }

    if !current_line.is_empty() {
        lines.push(current_line);
    }

    lines
}
