use crate::app::{App, InputMode};
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

pub fn handle_palette_mode(app: &mut App, key: KeyEvent) {
    match key.code {
        KeyCode::Esc => {
            app.input_mode = InputMode::Normal;
        }
        KeyCode::Char('c') if key.modifiers.contains(KeyModifiers::CONTROL) => {
            app.input_mode = InputMode::Normal;
        }
        KeyCode::Up | KeyCode::BackTab => {
            app.command_palette.selected_idx = app.command_palette.selected_idx.saturating_sub(1);
        }
        KeyCode::Char('p') | KeyCode::Char('k')
            if key.modifiers.contains(KeyModifiers::CONTROL) =>
        {
            app.command_palette.selected_idx = app.command_palette.selected_idx.saturating_sub(1);
        }
        KeyCode::Down | KeyCode::Tab => {
            let filtered_len = crate::palette::filter_commands(&app.command_palette.query).len();
            if filtered_len > 0 {
                app.command_palette.selected_idx =
                    (app.command_palette.selected_idx + 1).min(filtered_len.saturating_sub(1));
            }
        }
        KeyCode::Char('n' | 'j') if key.modifiers.contains(KeyModifiers::CONTROL) => {
            let filtered_len = crate::palette::filter_commands(&app.command_palette.query).len();
            if filtered_len > 0 {
                app.command_palette.selected_idx =
                    (app.command_palette.selected_idx + 1).min(filtered_len.saturating_sub(1));
            }
        }
        KeyCode::Enter => {
            let filtered = crate::palette::filter_commands(&app.command_palette.query);
            if let Some((cmd, _)) = filtered.get(app.command_palette.selected_idx) {
                let action = cmd.execute;
                app.input_mode = InputMode::Normal;
                action(app);
            }
        }
        KeyCode::Backspace => {
            app.command_palette.query.pop();
            app.command_palette.selected_idx = 0;
        }
        KeyCode::Char('u') if key.modifiers.contains(KeyModifiers::CONTROL) => {
            app.command_palette.query.clear();
            app.command_palette.selected_idx = 0;
        }
        KeyCode::Char(c) if !key.modifiers.contains(KeyModifiers::CONTROL) => {
            app.command_palette.query.push(c);
            app.command_palette.selected_idx = 0;
        }
        _ => {}
    }
}
