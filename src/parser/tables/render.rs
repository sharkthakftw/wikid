use super::types::{CellEntry, CellLinkInfo, TableGrid};
use crate::parser::types::{Link, ParsedDocument};
use crate::theme;
use ratatui::style::Style;
use ratatui::text::{Line, Span};
use unicode_width::UnicodeWidthStr;

pub fn render_grid(
    grid: &TableGrid,
    col_widths: &[usize],
    row_heights: &[usize],
    mut origin_lines: Vec<Vec<Vec<Span<'static>>>>,
    mut origin_links: Vec<Vec<CellLinkInfo>>,
    doc: &mut ParsedDocument,
) {
    let num_rows = grid.num_rows;
    let num_cols = grid.num_cols;

    let mut cell_rendered_lines: Vec<Vec<Vec<Span<'static>>>> =
        vec![Vec::new(); num_rows * num_cols];
    let mut cell_rendered_links: Vec<Vec<CellLinkInfo>> = vec![Vec::new(); num_rows * num_cols];

    for r in 0..num_rows {
        for c in 0..num_cols {
            let idx = r * num_cols + c;
            if let CellEntry::Origin { rowspan, .. } = &grid.cells[r][c] {
                if *rowspan == 1 {
                    cell_rendered_lines[idx] = std::mem::take(&mut origin_lines[idx]);
                    cell_rendered_links[idx] = std::mem::take(&mut origin_links[idx]);
                } else {
                    let all_lines = &origin_lines[idx];
                    let all_links = &origin_links[idx];
                    let total_slots: usize =
                        (0..*rowspan).filter_map(|dr| row_heights.get(r + dr)).sum();
                    let top_pad = total_slots.saturating_sub(all_lines.len()) / 2;
                    let mut cursor = 0;
                    let mut slot = 0;

                    for dr in 0..*rowspan {
                        let curr_r = r + dr;
                        if curr_r >= num_rows {
                            break;
                        }
                        let curr_idx = curr_r * num_cols + c;
                        let mut chunk = Vec::new();
                        let mut chunk_links = Vec::new();

                        for local_line in 0..row_heights[curr_r] {
                            if slot >= top_pad && cursor < all_lines.len() {
                                chunk.push(all_lines[cursor].clone());
                                for (target, text, coords) in all_links {
                                    let matched: Vec<_> = coords
                                        .iter()
                                        .filter(|(src_l, _)| *src_l == cursor)
                                        .map(|(_, src_s)| (local_line, *src_s))
                                        .collect();
                                    if !matched.is_empty() {
                                        chunk_links.push((target.clone(), text.clone(), matched));
                                    }
                                }
                                cursor += 1;
                            } else {
                                chunk.push(Vec::new());
                            }
                            slot += 1;
                        }
                        cell_rendered_lines[curr_idx] = chunk;
                        cell_rendered_links[curr_idx] = chunk_links;
                    }
                }
            }
        }
    }

    let border_style = Style::default().fg(theme::DARK_GREY);

    let mut top_spans = vec![Span::styled("┌", border_style)];
    let mut c = 0;
    while c < num_cols {
        let colspan = match &grid.cells[0][c] {
            CellEntry::Origin { colspan, .. } => *colspan,
            _ => 1,
        };
        let mut span_w = (0..colspan)
            .filter_map(|dc| col_widths.get(c + dc))
            .sum::<usize>();
        span_w += (colspan - 1) * 3;
        top_spans.push(Span::styled("─".repeat(span_w + 2), border_style));
        c += colspan;
        if c < num_cols {
            top_spans.push(Span::styled("┬", border_style));
        }
    }
    top_spans.push(Span::styled("┐", border_style));
    doc.lines.push(Line::from(top_spans));

    for r in 0..num_rows {
        let start_idx = doc.lines.len();
        let mut cell_span_starts = vec![0usize; row_heights[r] * num_cols];

        for line_in_row in 0..row_heights[r] {
            let mut line_spans = vec![Span::styled("│ ", border_style)];
            let mut c = 0;
            while c < num_cols {
                let (orig_c, colspan) = match &grid.cells[r][c] {
                    CellEntry::Origin { colspan, .. } => (c, *colspan),
                    CellEntry::Covered { origin_c, .. } => {
                        let span = match &grid.cells[r][*origin_c] {
                            CellEntry::Origin { colspan, .. } => *colspan,
                            _ => 1,
                        };
                        (*origin_c, span)
                    }
                };

                let mut span_w = (0..colspan)
                    .filter_map(|dc| col_widths.get(c + dc))
                    .sum::<usize>();
                span_w += (colspan - 1) * 3;

                let empty = Vec::new();
                let cell_spans = cell_rendered_lines
                    .get(r * num_cols + orig_c)
                    .and_then(|lines| lines.get(line_in_row))
                    .unwrap_or(&empty);

                let content_len: usize = cell_spans
                    .iter()
                    .map(|s| UnicodeWidthStr::width(s.content.as_ref()))
                    .sum();
                let padding = span_w.saturating_sub(content_len);

                cell_span_starts[line_in_row * num_cols + orig_c] = line_spans.len();

                for span in cell_spans {
                    line_spans.push(span.clone());
                }
                if padding > 0 {
                    line_spans.push(Span::raw(" ".repeat(padding)));
                }

                c += colspan;
                if c < num_cols {
                    line_spans.push(Span::styled(" │ ", border_style));
                } else {
                    line_spans.push(Span::styled(" │", border_style));
                }
            }
            doc.lines.push(Line::from(line_spans));
        }

        let mut row_links = Vec::new();
        for col_i in 0..num_cols {
            if let Some(links) = cell_rendered_links.get(r * num_cols + col_i) {
                for (target, text, coords) in links {
                    let mut span_indices = Vec::new();
                    for &(local_l, local_s) in coords {
                        if local_l < row_heights[r] {
                            let abs_l = start_idx + local_l;
                            let span_start = cell_span_starts[local_l * num_cols + col_i];
                            span_indices.push((abs_l, span_start + local_s));
                        }
                    }
                    if !span_indices.is_empty() {
                        row_links.push(Link {
                            title: target.clone(),
                            text: text.clone(),
                            span_indices,
                        });
                    }
                }
            }
        }
        row_links.sort_by_key(|link| link.span_indices[0]);
        doc.links.extend(row_links);

        if r + 1 < num_rows {
            let is_header_sep = grid.cells[r].iter().any(|cell| match cell {
                CellEntry::Origin { is_header, .. } => *is_header,
                _ => false,
            });
            let sep_char = if is_header_sep { "═" } else { "─" };

            let col_0_vert = match &grid.cells[r + 1][0] {
                CellEntry::Covered { origin_r, .. } => *origin_r <= r,
                _ => false,
            };
            let left_joint = if col_0_vert { "│" } else { "├" };
            let mut sep_spans = vec![Span::styled(left_joint, border_style)];

            let mut col = 0;
            while col < num_cols {
                let is_vert_cont = match &grid.cells[r + 1][col] {
                    CellEntry::Covered { origin_r, .. } => *origin_r <= r,
                    _ => false,
                };
                let colspan = match &grid.cells[r][col] {
                    CellEntry::Origin { colspan, .. } => *colspan,
                    CellEntry::Covered { origin_c, .. } => match &grid.cells[r][*origin_c] {
                        CellEntry::Origin { colspan, .. } => *colspan,
                        _ => 1,
                    },
                };
                let mut span_w = (0..colspan)
                    .filter_map(|dc| col_widths.get(col + dc))
                    .sum::<usize>();
                span_w += (colspan - 1) * 3;

                if is_vert_cont {
                    sep_spans.push(Span::raw(" ".repeat(span_w + 2)));
                } else {
                    sep_spans.push(Span::styled(sep_char.repeat(span_w + 2), border_style));
                }

                col += colspan;
                if col < num_cols {
                    let next_is_vert = match &grid.cells[r + 1][col] {
                        CellEntry::Covered { origin_r, .. } => *origin_r <= r,
                        _ => false,
                    };
                    let sep_joint = match (is_vert_cont, next_is_vert) {
                        (true, true) => "│",
                        (true, false) => {
                            if is_header_sep {
                                "╞"
                            } else {
                                "├"
                            }
                        }
                        (false, true) => {
                            if is_header_sep {
                                "╡"
                            } else {
                                "┤"
                            }
                        }
                        (false, false) => {
                            if is_header_sep {
                                "╪"
                            } else {
                                "┼"
                            }
                        }
                    };
                    sep_spans.push(Span::styled(sep_joint, border_style));
                }
            }
            let col_last_vert = match &grid.cells[r + 1][num_cols - 1] {
                CellEntry::Covered { origin_r, .. } => *origin_r <= r,
                _ => false,
            };
            let right_joint = if col_last_vert { "│" } else { "┤" };
            sep_spans.push(Span::styled(right_joint, border_style));
            doc.lines.push(Line::from(sep_spans));
        }
    }

    let mut bot_spans = vec![Span::styled("└", border_style)];
    let mut c = 0;
    while c < num_cols {
        let colspan = match &grid.cells[num_rows - 1][c] {
            CellEntry::Origin { colspan, .. } => *colspan,
            _ => 1,
        };
        let mut span_w = (0..colspan)
            .filter_map(|dc| col_widths.get(c + dc))
            .sum::<usize>();
        span_w += (colspan - 1) * 3;
        bot_spans.push(Span::styled("─".repeat(span_w + 2), border_style));
        c += colspan;
        if c < num_cols {
            bot_spans.push(Span::styled("┴", border_style));
        }
    }
    bot_spans.push(Span::styled("┘", border_style));
    doc.lines.push(Line::from(bot_spans));
    doc.lines.push(Line::from(""));
}
