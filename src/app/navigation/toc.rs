use crate::app::pane::PaneContent;
use crate::app::App;

impl App {
    pub fn toggle_toc(&mut self) {
        let pane = self.active_pane_mut();
        let has_headings = match &pane.content {
            PaneContent::ArticleText { parsed_doc, .. } => !parsed_doc.headings.is_empty(),
            _ => false,
        };

        if !has_headings {
            pane.show_toc = false;
            pane.toc_focused = false;
            return;
        }

        pane.show_toc = !pane.show_toc;
        pane.toc_focused = pane.show_toc;

        if pane.show_toc {
            let current_scroll = pane.scroll_offset;
            if let PaneContent::ArticleText { parsed_doc, .. } = &pane.content {
                let active_idx = parsed_doc
                    .headings
                    .iter()
                    .rposition(|h| h.line_idx <= current_scroll)
                    .unwrap_or(0);
                pane.selected_toc_idx = Some(active_idx);
            }
        }
    }

    pub fn select_next_toc_item(&mut self) {
        let pane = self.active_pane_mut();
        if let PaneContent::ArticleText { parsed_doc, .. } = &pane.content {
            if parsed_doc.headings.is_empty() {
                return;
            }
            let len = parsed_doc.headings.len();
            let next_idx = match pane.selected_toc_idx {
                Some(idx) => (idx + 1).min(len - 1),
                None => 0,
            };
            pane.selected_toc_idx = Some(next_idx);
        }
    }

    pub fn select_prev_toc_item(&mut self) {
        let pane = self.active_pane_mut();
        if let PaneContent::ArticleText { parsed_doc, .. } = &pane.content {
            if parsed_doc.headings.is_empty() {
                return;
            }
            let prev_idx = match pane.selected_toc_idx {
                Some(idx) => idx.saturating_sub(1),
                None => 0,
            };
            pane.selected_toc_idx = Some(prev_idx);
        }
    }

    pub fn activate_toc_selection(&mut self, term_height: u16) {
        let pane = self.active_pane_mut();
        let target_line = match (&pane.content, pane.selected_toc_idx) {
            (PaneContent::ArticleText { parsed_doc, .. }, Some(idx)) => {
                parsed_doc.headings.get(idx).map(|h| h.line_idx)
            }
            _ => None,
        };
        if let Some(line) = target_line {
            let cur = pane.scroll_offset;
            pane.intra_jump_back.push(cur);
            pane.intra_jump_forward.clear();
            pane.scroll_offset = line;
        }
        pane.show_toc = false;
        pane.toc_focused = false;
        self.clamp_link_selection_to_viewport(term_height);
    }

    pub fn set_status_message(&mut self, msg: impl Into<String>) {
        self.status_message.set(msg);
    }
}
