use ratatui::{
    DefaultTerminal, Frame,
    crossterm::event::{self, Event},
};
use rusqlite::Connection;

use crate::{
    app::{
        handlers::{handle_add_new_task, handle_key_press},
        renderers::{render_main_layout, render_popup},
    },
    db::get_tasks,
    error::AppError,
    status::Status,
};

#[derive(Debug, Clone)]
pub struct Task {
    pub id: i32,
    pub title: String,
    pub status: Status,
    pub created_at: i64,
    pub updated_at: i64,
}

#[derive(Debug, PartialEq, Eq)]
pub enum Mode {
    Add,
    Default,
}

#[derive(Debug)]
pub struct App {
    pub conn: Connection,
    pub exit: bool,
    pub mode: Mode,
    pub title_buf: String,
    pub tasks: Vec<Task>,
    pub update_tasks: bool,
    pub selected: Option<(u32, u32)>,
}

impl App {
    pub fn new(conn: Connection) -> Self {
        Self {
            conn,
            exit: false,
            mode: Mode::Default,
            title_buf: String::new(),
            tasks: vec![],
            update_tasks: true,
            selected: None,
        }
    }

    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> Result<(), AppError> {
        while !self.exit {
            if self.update_tasks {
                self.update_tasks()?;
                self.update_tasks = false;
            }

            terminal
                .draw(|frame| self.draw(frame))
                .map_err(AppError::IOError)?;
            self.handle_events()?;
        }
        Ok(())
    }

    fn draw(&self, frame: &mut Frame) {
        if self.mode == Mode::Add {
            render_popup(frame, &self.title_buf);
            return;
        }

        render_main_layout(frame, &self);
    }

    fn handle_events(&mut self) -> Result<(), AppError> {
        if self.mode == Mode::Add {
            match event::read().map_err(AppError::IOError)? {
                Event::Key(key_event) => {
                    handle_add_new_task(key_event, self)?;
                }
                _ => {}
            };
            return Ok(());
        }
        match event::read().map_err(AppError::IOError)? {
            Event::Key(key_event) => {
                handle_key_press(key_event, self);
            }
            _ => {}
        };
        Ok(())
    }

    pub fn update_tasks(&mut self) -> Result<(), AppError> {
        let tasks = get_tasks(&self.conn)?;
        self.tasks = tasks;
        Ok(())
    }
}
