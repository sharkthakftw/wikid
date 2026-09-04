use super::utils::render_modal_container_at;
use crate::app::App;
use crate::theme;
use ratatui::{
    layout::{Alignment, Rect},
    style::{Style, Stylize},
    text::{Line, Span},
    widgets::Paragraph,
    Frame,
};

pub fn compute_qr_modal_area(size: Rect) -> Rect {
    let width = 48.min(size.width.saturating_sub(2));
    let height = 26.min(size.height.saturating_sub(2));
    let x = (size.width.saturating_sub(width)) / 2;
    let y = (size.height.saturating_sub(height)) / 2;
    Rect {
        x,
        y,
        width,
        height,
    }
}

pub fn render_qr_modal(f: &mut Frame, app: &App, size: Rect) {
    let Some(qr_state) = &app.qr_modal else {
        return;
    };

    let icon = if app.config.ui.icons { "" } else { "" };
    let area = compute_qr_modal_area(size);
    let inner = render_modal_container_at(
        f,
        area,
        icon,
        "qr code",
        theme::PINK,
        app.config.ui.rounded_borders,
    );

    let n = qr_state.matrix.len();
    let quiet_zone = 2;
    let total_w = n + quiet_zone * 2;
    let total_h = n + quiet_zone * 2;

    let is_dark = |x: usize, y: usize| -> bool {
        if x < quiet_zone || x >= quiet_zone + n || y < quiet_zone || y >= quiet_zone + n {
            false
        } else {
            qr_state.matrix[y - quiet_zone][x - quiet_zone]
        }
    };

    let mut lines = Vec::new();

    let title_line = Line::from(vec![Span::styled(
        format!(" {} ", qr_state.title.to_lowercase()),
        Style::default().fg(theme::PINK).bold(),
    )])
    .alignment(Alignment::Center);
    lines.push(title_line);

    let pad_left = (inner.width as usize).saturating_sub(total_w) / 2;

    for y in (0..total_h).step_by(2) {
        let mut spans = Vec::with_capacity(total_w + 1);
        if pad_left > 0 {
            spans.push(Span::styled(
                " ".repeat(pad_left),
                Style::default().bg(theme::BG),
            ));
        }
        for x in 0..total_w {
            let top = is_dark(x, y);
            let bot = if y + 1 < total_h {
                is_dark(x, y + 1)
            } else {
                false
            };
            let ch = match (top, bot) {
                (true, true) => '█',
                (true, false) => '▀',
                (false, true) => '▄',
                (false, false) => ' ',
            };
            spans.push(Span::styled(
                ch.to_string(),
                Style::default().fg(theme::PINK).bg(theme::BG),
            ));
        }
        lines.push(Line::from(spans));
    }

    let url_display = if qr_state.full_url.len() > inner.width as usize {
        format!("{}…", &qr_state.full_url[..inner.width.saturating_sub(3) as usize])
    } else {
        qr_state.full_url.clone()
    };
    lines.push(
        Line::from(vec![Span::styled(
            url_display,
            Style::default().fg(theme::GREY),
        )])
        .alignment(Alignment::Center),
    );

    lines.push(
        Line::from(vec![
            Span::styled("esc/q", Style::default().fg(theme::YELLOW).bold()),
            Span::styled(" close   ", Style::default().fg(theme::GREY)),
            Span::styled("y", Style::default().fg(theme::YELLOW).bold()),
            Span::styled(" copy link", Style::default().fg(theme::GREY)),
        ])
        .alignment(Alignment::Center),
    );

    let p = Paragraph::new(lines).style(Style::default().bg(theme::BG));
    f.render_widget(p, inner);
}
