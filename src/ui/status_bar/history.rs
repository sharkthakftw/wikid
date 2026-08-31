use crate::theme;
use ratatui::style::{Modifier, Style};
use ratatui::text::Span;

pub fn build_history_trail(pane: &crate::app::Pane, available_width: usize) -> Vec<Span<'static>> {
    let mut spans = Vec::new();
    let back_len = pane.history_back.len();
    let start_idx = back_len.saturating_sub(2);
    let back_items = &pane.history_back[start_idx..];
    let fwd_items: Vec<_> = pane.history_forward.iter().take(2).collect();

    let total_items = 1 + back_items.len() + fwd_items.len();
    let overhead = total_items * 3
        + if start_idx > 0 { 4 } else { 0 }
        + if pane.history_forward.len() > 2 { 4 } else { 0 };
    let budget = available_width.saturating_sub(overhead);
    let side_max = (budget / total_items.max(1)).clamp(6, 35);
    let cur_max = (side_max + side_max / 3).clamp(8, 45);

    if start_idx > 0 {
        spans.push(Span::styled("…", Style::default().fg(theme::GREY)));
        spans.push(Span::styled(" › ", Style::default().fg(theme::GREY)));
    }

    for title in back_items {
        spans.push(Span::styled(
            truncate_trail_title(title, side_max),
            Style::default().fg(theme::GREY),
        ));
        spans.push(Span::styled(" › ", Style::default().fg(theme::GREY)));
    }

    if let Some(cur_title) = pane.title() {
        spans.push(Span::styled(
            truncate_trail_title(&cur_title, cur_max),
            Style::default()
                .fg(theme::LIME)
                .add_modifier(Modifier::BOLD),
        ));
    }

    for title in fwd_items {
        spans.push(Span::styled(" › ", Style::default().fg(theme::GREY)));
        spans.push(Span::styled(
            truncate_trail_title(title, side_max),
            Style::default()
                .fg(theme::GREY)
                .add_modifier(Modifier::ITALIC),
        ));
    }

    if pane.history_forward.len() > 2 {
        spans.push(Span::styled(" › …", Style::default().fg(theme::GREY)));
    }

    spans
}

fn truncate_trail_title(title: &str, max_len: usize) -> String {
    let lower = title.to_lowercase();
    crate::ui::truncate_with_ellipsis(&lower, max_len, "…")
}
