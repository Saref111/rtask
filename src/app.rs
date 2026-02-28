use std::io;

use ratatui::{
    DefaultTerminal, Frame,
    buffer::Buffer,
    crossterm::event::{self, Event, KeyCode},
    layout::{Constraint, Direction, Flex, Layout, Rect},
    style::Stylize,
    symbols::border,
    text::{Line, Text},
    widgets::{Block, Paragraph, Widget},
};
use rusqlite::Connection;

#[derive(Debug)]
pub struct App {
    conn: Connection,
    exit: bool,
}

impl App {
    pub fn new(conn: Connection) -> Self {
        Self { conn, exit: false }
    }

    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> io::Result<()> {
        while !self.exit {
            terminal.draw(|frame| self.draw(frame))?;
            self.handle_events()?;
        }
        Ok(())
    }

    fn draw(&self, frame: &mut Frame) {
        let title = Line::from(" Task Manager ".bold()).centered();

        let outer_layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints(vec![Constraint::Max(30), Constraint::Percentage(95)])
            .flex(Flex::SpaceBetween)
            .split(frame.area());

        let layout = Layout::default()
            .direction(Direction::Horizontal)
            .constraints(vec![
                Constraint::Percentage(30),
                Constraint::Percentage(30),
                Constraint::Percentage(30),
            ])
            .flex(Flex::SpaceBetween)
            .split(outer_layout[1]);
        frame.render_widget(title, outer_layout[0]);
        frame.render_widget(Column::new("To do".into()), layout[0]);
        frame.render_widget(Column::new("In process".into()), layout[1]);
        frame.render_widget(Column::new("Done".into()), layout[2]);
    }

    fn handle_events(&mut self) -> io::Result<()> {
        match event::read()? {
            Event::Key(key_event) => match key_event.code {
                KeyCode::Char('q') => self.exit = true,
                _ => {}
            },
            _ => {}
        };
        Ok(())
    }
}

struct Column {
    items: Vec<String>,
    name: String,
}

impl Column {
    pub fn new(name: String) -> Self {
        Self {
            items: vec![],
            name,
        }
    }
}

impl Widget for Column {
    fn render(self, area: Rect, buf: &mut Buffer)
    where
        Self: Sized,
    {
        let title = Line::from(self.name.as_str().bold());
        let block = Block::bordered().title(title.centered());
        block.render(area, buf);
    }
}

impl Widget for &App {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let instructions = Line::from(vec![
            " Decrement ".into(),
            "<Left>".blue().bold(),
            " Increment ".into(),
            "<Right>".blue().bold(),
            " Quit ".into(),
            "<Q> ".blue().bold(),
        ]);
    }
}
