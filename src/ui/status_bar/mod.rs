mod audio;
mod hints;
mod history;

use crate::app::{App, PaneContent};
use crate::theme;
use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::Paragraph,
    Frame,
};

pub use hints::{get_center_spans, get_mode_badge};

pub fn render(f: &mut Frame, app: &App, area: Rect) {
    if area.width < 10 || area.height == 0 {
        return;
    }

    let active_pane = app.active_pane();

    let (badge_text, badge_color) = get_mode_badge(app);
    let left_spans = vec![Span::styled(
        badge_text,
        Style::default()
            .fg(theme::BG)
            .bg(badge_color)
            .add_modifier(Modifier::BOLD),
    )];
    let left_width = badge_text.chars().count() as u16 + 2;

    let right_text = get_right_segment(active_pane);
    let right_spans = vec![Span::styled(
        format!(" {} ", right_text),
        Style::default()
            .fg(theme::GREY)
            .add_modifier(Modifier::ITALIC),
    )];
    let right_width = right_text.chars().count() as u16 + 3;

    let center_width = (area.width as usize).saturating_sub((left_width + right_width) as usize);

    let center_spans = if let Some((ref msg, time)) = app.status_message {
        if time.elapsed().as_secs_f32() < 3.0 {
            vec![Span::styled(
                msg.clone(),
                Style::default()
                    .fg(theme::LIME)
                    .add_modifier(Modifier::BOLD),
            )]
        } else {
            get_center_spans(app, active_pane, center_width)
        }
    } else {
        get_center_spans(app, active_pane, center_width)
    };

    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Length(left_width),
            Constraint::Min(0),
            Constraint::Length(right_width),
        ])
        .split(area);

    f.render_widget(
        Paragraph::new(Line::from(left_spans)).alignment(Alignment::Left),
        chunks[0],
    );
    f.render_widget(
        Paragraph::new(Line::from(center_spans)).alignment(Alignment::Center),
        chunks[1],
    );
    f.render_widget(
        Paragraph::new(Line::from(right_spans)).alignment(Alignment::Right),
        chunks[2],
    );
}

fn get_right_segment(active_pane: &crate::app::Pane) -> String {
    match &active_pane.content {
        PaneContent::ArticleText { parsed_doc, .. } => {
            let total_lines = parsed_doc.lines.len();
            let scroll = active_pane.scroll_offset;
            let viewport = active_pane.viewport_height.max(1);
            let max_scroll = total_lines.saturating_sub(viewport);

            if total_lines <= viewport {
                "ALL".to_string()
            } else if scroll == 0 {
                "TOP".to_string()
            } else if scroll >= max_scroll {
                "BOT".to_string()
            } else {
                let pct = (((scroll as f64) / (max_scroll as f64)) * 100.0)
                    .round()
                    .clamp(1.0, 99.0) as usize;
                format!("{}%", pct)
            }
        }
        PaneContent::SearchResults { items, .. } => {
            if !items.is_empty() {
                format!("{}/{}", active_pane.selected_idx + 1, items.len())
            } else {
                String::new()
            }
        }
        PaneContent::Empty => {
            format!("v{}", env!("CARGO_PKG_VERSION"))
        }
        PaneContent::Error(_) => "ERR".to_string(),
    }
}
