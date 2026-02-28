use ratatui::crossterm::event::{KeyCode, KeyEvent};

use crate::app::{App, Mode};

pub fn handle_add_new_task(key_event: KeyEvent, app: &mut App) {
    match key_event.code {
        KeyCode::Char(c) => {
            if c.is_alphanumeric() || c.is_ascii_punctuation() || c.is_ascii_whitespace() {
                app.title_buf.push(c);
            }
        }
        KeyCode::Esc => {
            app.mode = Mode::Default;
            app.title_buf = String::new();
        }
        KeyCode::Enter => {
            app.mode = Mode::Default;
            // todo smth with title_buf
            app.title_buf = String::new();
        }
        _ => {}
    };
}

pub fn handle_key_press(key_event: KeyEvent, app: &mut App) {
    match key_event.code {
        KeyCode::Char('q') => app.exit = true,
        KeyCode::Char('a') => app.mode = Mode::Add,
        _ => {}
    }
}
