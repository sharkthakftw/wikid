use crate::app::App;
use crossterm::event::{KeyCode, KeyEvent};

pub fn handle_qr_mode(app: &mut App, key: KeyEvent) {
    match key.code {
        KeyCode::Esc | KeyCode::Char('q') | KeyCode::Enter => {
            app.close_qr_modal();
        }
        KeyCode::Char('y') => {
            app.copy_article_link();
        }
        _ => {}
    }
}
