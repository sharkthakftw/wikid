use crate::app::{App, InputMode};
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

pub fn handle_search_mode(app: &mut App, key: KeyEvent) {
    if key.modifiers.contains(KeyModifiers::CONTROL) {
        match key.code {
            KeyCode::Char('w') | KeyCode::Char('h') | KeyCode::Backspace => {
                app.delete_word_left();
                return;
            }
            _ => {}
        }
    }
    match key.code {
        KeyCode::Char(c) => {
            app.type_search_char(c);
        }
        KeyCode::Backspace => {
            app.backspace_search_char();
        }
        KeyCode::Delete => {
            app.delete_search_char();
        }
        KeyCode::Left => {
            app.move_search_cursor_left();
        }
        KeyCode::Right => {
            app.move_search_cursor_right();
        }
        KeyCode::Home => {
            app.move_search_cursor_home();
        }
        KeyCode::End => {
            app.move_search_cursor_end();
        }
        KeyCode::Enter => {
            app.submit_search();
        }
        KeyCode::Esc => {
            app.exit_search_mode();
        }
        _ => {}
    }
}

pub fn handle_local_search_mode(app: &mut App, key: KeyEvent, term_height: u16) {
    match key.code {
        KeyCode::Char(c) => {
            let pane = app.active_pane_mut();
            pane.search.query.push(c);
            app.update_local_search(term_height);
        }
        KeyCode::Backspace => {
            let pane = app.active_pane_mut();
            pane.search.query.pop();
            app.update_local_search(term_height);
        }
        KeyCode::Enter => {
            app.input_mode = InputMode::Normal;
        }
        KeyCode::Esc => {
            app.clear_local_search();
            app.input_mode = InputMode::Normal;
        }
        _ => {}
    }
}
