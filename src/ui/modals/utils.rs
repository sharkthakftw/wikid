use crate::theme;
use ratatui::{
    layout::{Alignment, Rect},
    style::{Color, Style, Stylize},
    text::Span,
    widgets::{block::Title, Block},
};

pub fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    let width = (r.width * percent_x.min(100)) / 100;
    let height = (r.height * percent_y.min(100)) / 100;
    let x = r.x + (r.width.saturating_sub(width)) / 2;
    let y = r.y + (r.height.saturating_sub(height)) / 2;
    Rect {
        x,
        y,
        width,
        height,
    }
}

pub fn compute_centered_scroll(
    cursor_idx: usize,
    visible_rows: usize,
    total_items: usize,
) -> usize {
    if total_items <= visible_rows || visible_rows == 0 {
        0
    } else {
        cursor_idx
            .saturating_sub(visible_rows / 2)
            .min(total_items.saturating_sub(visible_rows))
    }
}

pub fn create_modal_block(
    icon: &str,
    title: &str,
    border_color: Color,
    rounded: bool,
) -> Block<'static> {
    let top_title = if icon.is_empty() {
        format!(" {} ", title)
    } else {
        format!(" {} {} ", icon, title)
    };

    let border_type = theme::border_type(rounded);

    Block::bordered()
        .border_type(border_type)
        .border_style(Style::default().fg(border_color))
        .style(Style::default().bg(theme::BG))
        .title(
            Title::from(Span::styled(
                top_title,
                Style::default().fg(border_color).bold(),
            ))
            .alignment(Alignment::Center),
        )
}

pub fn render_modal_frame_at(
    f: &mut ratatui::Frame,
    area: Rect,
    icon: &str,
    title: &str,
    border_color: Color,
    rounded: bool,
) -> Block<'static> {
    f.render_widget(ratatui::widgets::Clear, area);
    create_modal_block(icon, title, border_color, rounded)
}

pub fn render_modal_container_at(
    f: &mut ratatui::Frame,
    area: Rect,
    icon: &str,
    title: &str,
    border_color: Color,
    rounded: bool,
) -> Rect {
    let block = render_modal_frame_at(f, area, icon, title, border_color, rounded);
    let inner = block.inner(area);
    f.render_widget(block, area);
    inner
}

pub fn create_checkbox_line(
    label: &str,
    is_focused: bool,
    is_checked: bool,
    suffix: Option<&str>,
    cursor_color: Color,
) -> ratatui::text::Line<'static> {
    let cursor_str = if is_focused { " ▶ " } else { "   " };
    let check_str = if is_checked { "[x] " } else { "[ ] " };

    let item_style = if is_focused {
        Style::default().fg(theme::YELLOW).bold()
    } else if is_checked {
        Style::default().fg(theme::LIME)
    } else {
        Style::default().fg(theme::FG)
    };

    let check_style = if is_checked {
        Style::default().fg(theme::LIME).bold()
    } else {
        Style::default().fg(theme::GREY)
    };

    let mut spans = vec![
        Span::styled(cursor_str, Style::default().fg(cursor_color).bold()),
        Span::styled(check_str, check_style),
        Span::styled(label.to_string(), item_style),
    ];

    if let Some(suf) = suffix {
        spans.push(Span::styled(
            suf.to_string(),
            Style::default().fg(theme::GREY),
        ));
    }

    ratatui::text::Line::from(spans)
}

pub fn create_selectable_line(
    label: &str,
    is_selected: bool,
    is_active: bool,
    cursor_color: Color,
    suffix: Option<&str>,
) -> ratatui::text::Line<'static> {
    let prefix = if is_selected { " ▶ " } else { "   " };
    let style = if is_selected && is_active {
        Style::default().fg(theme::YELLOW).bold()
    } else if is_selected {
        Style::default().fg(theme::VIOLET).bold()
    } else {
        Style::default().fg(theme::FG)
    };

    let mut spans = vec![
        Span::styled(prefix, Style::default().fg(cursor_color)),
        Span::styled(label.to_string(), style),
    ];

    if let Some(suf) = suffix {
        spans.push(Span::styled(
            suf.to_string(),
            Style::default().fg(theme::GREY),
        ));
    }

    ratatui::text::Line::from(spans)
}

pub fn create_search_input_lines(
    prompt: &str,
    query: &str,
    prompt_color: Color,
    inner_width: usize,
) -> (ratatui::text::Line<'static>, ratatui::text::Line<'static>) {
    let input_line = ratatui::text::Line::from(vec![
        Span::styled(
            format!(" {} ", prompt),
            Style::default().fg(prompt_color).bold(),
        ),
        Span::styled(query.to_string(), Style::default().fg(theme::FG).bold()),
        Span::styled("█", Style::default().fg(prompt_color)),
    ]);
    let divider_line = ratatui::text::Line::from(Span::styled(
        "─".repeat(inner_width),
        Style::default().fg(theme::DARK_GREY),
    ));
    (input_line, divider_line)
}
