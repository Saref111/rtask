use ratatui::{
    Frame,
    layout::{Constraint, Direction, Flex, Layout},
    style::Stylize,
    text::{Line, Text},
    widgets::{Block, Paragraph, Wrap},
};

use crate::{
    app::{Task, column::Column},
    status::Status,
};

pub fn render_popup(frame: &mut Frame, title_buf: &str) {
    let popup = Block::bordered()
        .title(Line::from("Enter task title: ").centered())
        .title_bottom(Line::from("Press <Enter> to add new task. Press <Esc> to exit").centered());

    let text = Text::from(vec![Line::from(title_buf)]);
    let popup = Paragraph::new(text).block(popup).wrap(Wrap { trim: true });

    frame.render_widget(
        popup,
        frame
            .area()
            .centered(Constraint::Ratio(1, 2), Constraint::Ratio(1, 2)),
    );
}

pub fn render_main_layout(frame: &mut Frame, tasks: &Vec<Task>) {
    let todo_tasks = tasks
        .iter()
        .filter_map(|t| {
            if t.status == Status::ToDo {
                return Some(t.clone());
            }
            None
        })
        .collect::<Vec<Task>>();
    let in_progress_tasks = tasks
        .iter()
        .filter_map(|t| {
            if t.status == Status::InProgress {
                return Some(t.clone());
            }
            None
        })
        .collect::<Vec<Task>>();
    let done_tasks = tasks
        .iter()
        .filter_map(|t| {
            if t.status == Status::Done {
                return Some(t.clone());
            }
            None
        })
        .collect::<Vec<Task>>();

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
    frame.render_widget(Column::new("To do".into(), todo_tasks), main_layout[0]);
    frame.render_widget(
        Column::new("In progress".into(), in_progress_tasks),
        main_layout[1],
    );
    frame.render_widget(Column::new("Done".into(), done_tasks), main_layout[2]);
}
