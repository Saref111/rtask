use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::Stylize,
    text::Line,
    widgets::{Block, Widget},
};

pub struct Column {
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
