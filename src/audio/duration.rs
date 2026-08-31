fn unit_multiplier(unit: &str) -> Option<u64> {
    match unit {
        u if u.starts_with("hour") || u.starts_with("hr") || u == "h" => Some(3600),
        u if u.starts_with("minute") || u.starts_with("min") || u == "m" => Some(60),
        u if u.starts_with("second") || u.starts_with("sec") || u == "s" => Some(1),
        _ => None,
    }
}

pub fn parse_duration_to_secs(dur_str: &str) -> Option<u64> {
    for token in dur_str.split_whitespace() {
        if token.contains(':') {
            let mut parts = token.split(':');
            match (parts.next(), parts.next(), parts.next(), parts.next()) {
                (Some(m), Some(s), None, _) => {
                    if let (Ok(m), Ok(s)) = (m.parse::<u64>(), s.parse::<u64>()) {
                        return Some(m * 60 + s);
                    }
                }
                (Some(h), Some(m), Some(s), None) => {
                    if let (Ok(h), Ok(m), Ok(s)) =
                        (h.parse::<u64>(), m.parse::<u64>(), s.parse::<u64>())
                    {
                        return Some(h * 3600 + m * 60 + s);
                    }
                }
                _ => {}
            }
        }
    }

    let clean: String = dur_str
        .chars()
        .map(|c| {
            if c.is_alphanumeric() || c == '.' {
                c.to_ascii_lowercase()
            } else {
                ' '
            }
        })
        .collect();

    let words: Vec<&str> = clean.split_whitespace().collect();
    let mut total_secs = 0u64;
    let mut found = false;
    let mut i = 0;

    while i < words.len() {
        let w = words[i];
        if let Ok(num) = w.parse::<u64>() {
            if let Some(mult) = words.get(i + 1).and_then(|u| unit_multiplier(u)) {
                total_secs += num * mult;
                found = true;
                i += 2;
                continue;
            }
        } else if let Some(pos) = w.find(|c: char| c.is_alphabetic()) {
            let (digits, unit) = w.split_at(pos);
            if let (Ok(num), Some(mult)) = (digits.parse::<u64>(), unit_multiplier(unit)) {
                total_secs += num * mult;
                found = true;
            }
        }
        i += 1;
    }

    if found && total_secs > 0 {
        Some(total_secs)
    } else {
        None
    }
}
