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

#[derive(Debug, PartialEq, Eq)]
enum Mode {
    Add,
    Default,
}

#[derive(Debug)]
pub struct App {
    conn: Connection,
    exit: bool,
    mode: Mode,
    title_buf: String,
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
            let popup = Block::bordered()
                .title(Line::from("Enter task title: ").centered())
                .title_bottom(Line::from("Press <Enter> to add new task").centered());

            let text = Text::from(vec![Line::from(vec![self.title_buf.as_str().into()])]);
            let popup = Paragraph::new(text).block(popup);

            frame.render_widget(
                popup,
                frame
                    .area()
                    .centered(Constraint::Ratio(1, 2), Constraint::Ratio(1, 2)),
            );
            return;
        }

        let title = Line::from(" Task Manager ".bold()).centered();

        let outer_layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints(vec![Constraint::Max(30), Constraint::Percentage(95)])
            .flex(Flex::SpaceBetween)
            .split(frame.area());

        let main_layout = Layout::default()
            .direction(Direction::Horizontal)
            .constraints(vec![
                Constraint::Percentage(30),
                Constraint::Percentage(30),
                Constraint::Percentage(30),
            ])
            .flex(Flex::SpaceBetween)
            .split(outer_layout[1]);

        frame.render_widget(title, outer_layout[0]);
        frame.render_widget(Column::new("To do".into()), main_layout[0]);
        frame.render_widget(Column::new("In progress".into()), main_layout[1]);
        frame.render_widget(Column::new("Done".into()), main_layout[2]);
    }

    fn handle_events(&mut self) -> io::Result<()> {
        if self.mode == Mode::Add {
            match event::read()? {
                Event::Key(key_event) => match key_event.code {
                    KeyCode::Char(c) => {
                        if c.is_alphanumeric()
                            || c.is_ascii_punctuation()
                            || c.is_ascii_whitespace()
                        {
                            self.title_buf.push(c);
                        }
                    }
                    KeyCode::Enter => self.mode = Mode::Default,
                    _ => {}
                },
                _ => {}
            };
            return Ok(());
        }
        match event::read()? {
            Event::Key(key_event) => match key_event.code {
                KeyCode::Char('q') => self.exit = true,
                KeyCode::Char('a') => self.mode = Mode::Add,
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
