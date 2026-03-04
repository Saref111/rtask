use std::fmt::Debug;

use chrono::{Datelike, SubsecRound, TimeZone, Utc};
use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Direction, HorizontalAlignment, Layout, Rect},
    style::Stylize,
    text::Line,
    widgets::{Block, List, ListItem, Paragraph, Widget},
};

use crate::app::Task;

pub struct Column {
    items: Vec<Task>,
    name: String,
}

impl Column {
    pub fn new(name: String, items: Vec<Task>) -> Self {
        Self { items, name }
    }
}

impl Widget for Column {
    fn render(self, area: Rect, buf: &mut Buffer)
    where
        Self: Sized,
    {
        let title = Line::from(self.name.as_str().bold());
        let block = if self.items.len() == 0 {
            Block::bordered().title(title.centered())
        } else {
            Block::new().title(title.centered())
        };

        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints(
                self.items
                    .iter()
                    .map(|_| Constraint::Length(3))
                    .collect::<Vec<_>>(),
            )
            .split(block.inner(area));

        for (chunk, t) in chunks.iter().zip(&self.items) {
            let last_update = Utc.timestamp_micros(t.updated_at).unwrap();
            let last_update = format!(
                "{}, {}, {}, {}",
                last_update.year(),
                if last_update.month() > 9 {
                    last_update.month().to_string()
                } else {
                    "0".to_owned() + last_update.month().to_string().as_str()
                },
                if last_update.day() > 9 {
                    last_update.day().to_string()
                } else {
                    "0".to_owned() + last_update.day().to_string().as_str()
                },
                last_update.time().round_subsecs(0)
            );

            let p = Paragraph::new(t.title.as_str()).block(
                Block::bordered()
                    .title_bottom(last_update)
                    .title_alignment(HorizontalAlignment::Right),
            );
            p.render(*chunk, buf);
        }
        block.render(area, buf);
    }
}
