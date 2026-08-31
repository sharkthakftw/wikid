use super::{App, InputMode, PaneContent};

impl App {
    pub fn open_save_to_list_modal(&mut self) {
        let pane = self.active_pane();
        let (title, snippet) = match &pane.content {
            PaneContent::ArticleText { title, .. } => (title.clone(), None),
            PaneContent::SearchResults { items, .. } => {
                if let Some(item) = items.get(pane.selected_idx) {
                    (item.title.clone(), Some(item.snippet.clone()))
                } else {
                    return;
                }
            }
            _ => return,
        };

        if title.trim().is_empty() {
            return;
        }

        self.saved_lists = crate::saved_lists::SavedListsStore::load();
        self.lists_modal.target_title = title;
        self.lists_modal.target_snippet = snippet;
        self.lists_modal.save_cursor_idx = 0;
        self.input_mode = InputMode::SaveToList;
    }

    pub fn open_saved_lists_viewer(&mut self) {
        self.saved_lists = crate::saved_lists::SavedListsStore::load();
        self.lists_modal.viewer_list_idx = 0;
        self.lists_modal.viewer_article_idx = 0;
        self.lists_modal.viewer_focus_right = false;
        self.input_mode = InputMode::SavedListsViewer;
    }

    pub fn submit_create_new_list(&mut self) {
        let name = self.lists_modal.create_input.trim().to_string();
        if !name.is_empty() {
            let list_id = self.saved_lists.create_list(&name);
            if !self.lists_modal.target_title.is_empty() {
                self.saved_lists
                    .toggle_article_in_list(&list_id, &self.lists_modal.target_title);
            }
        }
        self.lists_modal.create_input.clear();
        self.input_mode = self.lists_modal.create_return_mode.clone();
    }

    pub fn toggle_help_popup(&mut self) {
        if self.input_mode == InputMode::Help {
            self.input_mode = InputMode::Normal;
        } else {
            self.input_mode = InputMode::Help;
        }
    }

    pub fn toggle_categories_modal(&mut self) {
        if self.input_mode == InputMode::Categories {
            self.input_mode = InputMode::Normal;
            return;
        }

        if matches!(self.active_pane().content, PaneContent::ArticleText { .. }) {
            self.categories_modal.cursor_idx = 0;
            self.categories_modal.article_cursor_idx = 0;
            self.categories_modal.focus_right = false;
            self.input_mode = InputMode::Categories;

            let first_cat = if let PaneContent::ArticleText { parsed_doc, .. } = &self.active_pane().content {
                parsed_doc.categories.first().cloned()
            } else {
                None
            };
            if let Some(cat) = first_cat {
                self.fetch_category_members_if_needed(&cat);
            }
        }
    }
}
