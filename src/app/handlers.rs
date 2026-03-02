use ratatui::crossterm::event::{KeyCode, KeyEvent};

use crate::{
    app::{App, Mode},
    db::create_task,
    error::AppError,
};

pub fn handle_add_new_task(key_event: KeyEvent, app: &mut App) -> Result<(), AppError> {
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
            create_task(&app.conn, app.title_buf.to_owned())?;
            app.title_buf = String::new();
        }
        _ => {}
    };

    Ok(())
}

pub fn handle_key_press(key_event: KeyEvent, app: &mut App) {
    match key_event.code {
        KeyCode::Char('q') => app.exit = true,
        KeyCode::Char('a') => app.mode = Mode::Add,
        _ => {}
    }
}
