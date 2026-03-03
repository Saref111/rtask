use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::Stylize,
    text::Line,
    widgets::{Block, List, Widget},
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
        let block = Block::bordered().title(title.centered());
        let list = List::new(self.items.iter().map(|t| t.title.to_string())).block(block);

        list.render(area, buf);
    }
}
