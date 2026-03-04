use ratatui::crossterm::event::{KeyCode, KeyEvent};

use crate::{
    app::{App, Mode},
    db::create_task,
    error::AppError,
    status::Status,
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
        KeyCode::Backspace => {
            app.title_buf.pop();
        }
        KeyCode::Enter => {
            if app.title_buf.is_empty() {
                return Ok(());
            }
            app.mode = Mode::Default;
            create_task(&app.conn, app.title_buf.to_owned())?;
            app.update_tasks()?;
            app.title_buf = String::new();
        }
        _ => {}
    };

    Ok(())
}

pub fn handle_key_press(key_event: KeyEvent, app: &mut App) {
    let (todo_len, in_progress_len, done_len) =
        app.tasks.iter().fold((0, 0, 0), |acc, it| match it.status {
            Status::ToDo => (acc.0 + 1, acc.1, acc.2),
            Status::InProgress => (acc.0, acc.1 + 1, acc.2),
            Status::Done => (acc.0, acc.1, acc.2 + 1),
        });

    match key_event.code {
        KeyCode::Char('q') => app.exit = true,
        KeyCode::Char('a') => app.mode = Mode::Add,
        KeyCode::Esc => app.selected = None,
        KeyCode::Right => {
            if todo_len == 0 && in_progress_len == 0 && done_len == 0 {
                return;
            }
            match app.selected {
                None => {
                    if todo_len > 0 {
                        app.selected = Some((0, 0));
                    } else if in_progress_len > 0 {
                        app.selected = Some((1, 0));
                    } else if done_len > 0 {
                        app.selected = Some((2, 0));
                    }
                }
                Some((c, _)) => {
                    if c == 2 {
                        return;
                    }

                    if c == 0 {
                        if in_progress_len > 0 {
                            app.selected = Some((1, 0));
                        } else if done_len > 0 {
                            app.selected = Some((2, 0));
                        }
                    }

                    if c == 1 {
                        if done_len > 0 {
                            app.selected = Some((2, 0));
                        }
                    }
                }
            }
        }
        KeyCode::Left => {
            if todo_len == 0 && in_progress_len == 0 && done_len == 0 {
                return;
            }

            match app.selected {
                None => {
                    if done_len > 0 {
                        app.selected = Some((2, 0));
                    } else if in_progress_len > 0 {
                        app.selected = Some((1, 0));
                    } else if todo_len > 0 {
                        app.selected = Some((0, 0));
                    }
                }
                Some((c, _)) => {
                    if c == 0 {
                        return;
                    }

                    if c == 2 {
                        if in_progress_len > 0 {
                            app.selected = Some((1, 0));
                        } else if todo_len > 0 {
                            app.selected = Some((0, 0));
                        }
                    }

                    if c == 1 {
                        if todo_len > 0 {
                            app.selected = Some((0, 0));
                        }
                    }
                }
            }
        }
        KeyCode::Down => {
            if todo_len == 0 && in_progress_len == 0 && done_len == 0 {
                return;
            }
        }
        KeyCode::Up => {
            if todo_len == 0 && in_progress_len == 0 && done_len == 0 {
                return;
            }
        }
        _ => {}
    }
}
