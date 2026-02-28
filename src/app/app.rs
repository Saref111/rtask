use std::io;

use ratatui::{
    DefaultTerminal, Frame,
    crossterm::event::{self, Event},
};
use rusqlite::Connection;

use crate::app::{
    handlers::{handle_add_new_task, handle_key_press},
    renderers::{render_main_layout, render_popup},
};

#[derive(Debug, PartialEq, Eq)]
pub enum Mode {
    Add,
    Default,
}

#[derive(Debug)]
pub struct App {
    conn: Connection,
    pub exit: bool,
    pub mode: Mode,
    pub title_buf: String,
}

impl App {
    pub fn new(conn: Connection) -> Self {
        Self {
            conn,
            exit: false,
            mode: Mode::Default,
            title_buf: String::new(),
        }
    }

    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> io::Result<()> {
        while !self.exit {
            terminal.draw(|frame| self.draw(frame))?;
            self.handle_events()?;
        }
        Ok(())
    }

    fn draw(&self, frame: &mut Frame) {
        if self.mode == Mode::Add {
            render_popup(frame, &self.title_buf);
            return;
        }

        render_main_layout(frame);
    }

    fn handle_events(&mut self) -> io::Result<()> {
        if self.mode == Mode::Add {
            match event::read()? {
                Event::Key(key_event) => {
                    handle_add_new_task(key_event, self);
                }
                _ => {}
            };
            return Ok(());
        }
        match event::read()? {
            Event::Key(key_event) => {
                handle_key_press(key_event, self);
            }
            _ => {}
        };
        Ok(())
    }
}
