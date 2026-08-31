use super::utils::{centered_rect, create_modal_block, create_selectable_line};
use crate::app::{App, PaneContent};
use crate::theme;
use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Style, Stylize},
    text::{Line, Span},
    widgets::Paragraph,
    Frame,
};

pub fn compute_categories_modal_areas(size: Rect) -> (Rect, Rect, Rect) {
    let container_area = centered_rect(80, 75, size);
    let inner_area = Rect::new(
        container_area.x + 1,
        container_area.y + 1,
        container_area.width.saturating_sub(2),
        container_area.height.saturating_sub(2),
    );
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(35), Constraint::Percentage(65)])
        .split(inner_area);
    (container_area, chunks[0], chunks[1])
}

pub fn render_categories_modal(f: &mut Frame, app: &App, size: Rect) {
    let pane = app.active_pane();
    let (title, categories) = match &pane.content {
        PaneContent::ArticleText {
            title, parsed_doc, ..
        } => (title.as_str(), &parsed_doc.categories),
        _ => return,
    };

    let total = categories.len();
    let modal_title = format!("categories · {} ({})", title.to_lowercase(), total);
    let icon = if app.config.ui.icons { "󰓹" } else { "" };

    let (container_area, left_area, right_area) = compute_categories_modal_areas(size);
    f.render_widget(ratatui::widgets::Clear, container_area);
    let block = create_modal_block(
        icon,
        &modal_title,
        theme::TEAL,
        app.config.ui.rounded_borders,
    );
    f.render_widget(block, container_area);

    let left_border_color = if !app.categories_modal.focus_right {
        theme::TEAL
    } else {
        theme::GREY
    };
    let left_block = create_modal_block(
        "",
        "categories",
        left_border_color,
        app.config.ui.rounded_borders,
    );

    let selected_cat_idx = app
        .categories_modal
        .cursor_idx
        .min(total.saturating_sub(1));
    let left_visible_rows = (left_area.height.saturating_sub(2)) as usize;
    let left_scroll = crate::ui::modals::utils::compute_centered_scroll(
        selected_cat_idx,
        left_visible_rows,
        total,
    );

    let mut cat_lines = Vec::new();
    if categories.is_empty() {
        cat_lines.push(Line::from(vec![Span::styled(
            "  no categories found for this article.",
            Style::default().fg(theme::GREY).italic(),
        )]));
    } else {
        for (idx, cat) in categories.iter().enumerate() {
            let is_selected = idx == selected_cat_idx;
            cat_lines.push(create_selectable_line(
                cat,
                is_selected,
                !app.categories_modal.focus_right,
                theme::TEAL,
                None,
            ));
        }
    }

    let left_paragraph = Paragraph::new(cat_lines)
        .block(left_block)
        .scroll((left_scroll as u16, 0));
    f.render_widget(left_paragraph, left_area);

    let selected_category = categories.get(selected_cat_idx).map(|s| s.as_str()).unwrap_or("");
    let right_title = if selected_category.is_empty() {
        "articles".to_string()
    } else {
        format!("articles in {}", selected_category.to_lowercase())
    };
    let right_border_color = if app.categories_modal.focus_right {
        theme::TEAL
    } else {
        theme::GREY
    };
    let right_block = create_modal_block(
        "",
        &right_title,
        right_border_color,
        app.config.ui.rounded_borders,
    );

    let mut article_lines = Vec::new();
    let right_visible_rows = (right_area.height.saturating_sub(2)) as usize;
    let mut right_scroll = 0;

    if categories.is_empty() {
        article_lines.push(Line::from(vec![Span::styled(
            "  no category selected.",
            Style::default().fg(theme::GREY).italic(),
        )]));
    } else if app
        .categories_modal
        .fetching_categories
        .contains(selected_category)
    {
        article_lines.push(Line::from(vec![Span::styled(
            "  fetching articles in category...",
            Style::default().fg(theme::YELLOW).italic(),
        )]));
    } else if let Some(members) = app.categories_modal.cached_members.get(selected_category) {
        if members.is_empty() {
            article_lines.push(Line::from(vec![Span::styled(
                "  no articles found in this category.",
                Style::default().fg(theme::GREY).italic(),
            )]));
        } else {
            let total_members = members.len();
            let selected_art_idx = app
                .categories_modal
                .article_cursor_idx
                .min(total_members.saturating_sub(1));
            right_scroll = crate::ui::modals::utils::compute_centered_scroll(
                selected_art_idx,
                right_visible_rows,
                total_members,
            );

            for (idx, member) in members.iter().enumerate() {
                let is_selected = idx == selected_art_idx;
                article_lines.push(create_selectable_line(
                    member,
                    is_selected,
                    app.categories_modal.focus_right,
                    theme::TEAL,
                    None,
                ));
            }
        }
    } else {
        article_lines.push(Line::from(vec![Span::styled(
            "  loading articles...",
            Style::default().fg(theme::GREY).italic(),
        )]));
    }

    let right_paragraph = Paragraph::new(article_lines)
        .block(right_block)
        .scroll((right_scroll as u16, 0));
    f.render_widget(right_paragraph, right_area);
}

pub fn get_category_item_at(
    app: &App,
    is_right: bool,
    area: Rect,
    target_y: u16,
) -> Option<usize> {
    if target_y <= area.y || target_y >= area.y + area.height.saturating_sub(1) {
        return None;
    }
    let row_offset = (target_y - (area.y + 1)) as usize;
    let visible_rows = (area.height.saturating_sub(2)) as usize;
    if is_right {
        let pane = app.active_pane();
        let categories = match &pane.content {
            PaneContent::ArticleText { parsed_doc, .. } => &parsed_doc.categories,
            _ => return None,
        };
        let selected_cat_idx = app
            .categories_modal
            .cursor_idx
            .min(categories.len().saturating_sub(1));
        let selected_category = categories.get(selected_cat_idx)?;
        let members = app.categories_modal.cached_members.get(selected_category)?;
        let total = members.len();
        let scroll = crate::ui::modals::utils::compute_centered_scroll(
            app.categories_modal.article_cursor_idx,
            visible_rows,
            total,
        );
        let idx = scroll + row_offset;
        if idx < total {
            Some(idx)
        } else {
            None
        }
    } else {
        let pane = app.active_pane();
        let total = match &pane.content {
            PaneContent::ArticleText { parsed_doc, .. } => parsed_doc.categories.len(),
            _ => 0,
        };
        let scroll = crate::ui::modals::utils::compute_centered_scroll(
            app.categories_modal.cursor_idx,
            visible_rows,
            total,
        );
        let idx = scroll + row_offset;
        if idx < total {
            Some(idx)
        } else {
            None
        }
    }
}
