use crate::app::App;

use tui::{
    backend::{Backend, CrosstermBackend},
    layout::{Constraint, Corner, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Span, Spans},
    widgets::{Block, Borders, List, ListItem},
    Frame, Terminal,
};

pub fn ui<B: Backend>(f: &mut Frame<B>, app: &mut App) {
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(100)].as_ref())
        .split(f.size());

    let items: Vec<ListItem> = app
        .todos
        .items
        .iter()
        .map(|i| {
            let mut lines = vec![Spans::from(i.message.clone())];
            lines.push(Spans::from(Span::styled(
                match i.complete {
                    0 => "complete",
                    1 => "incomplete",
                    _ => "error",
                },
                Style::default().add_modifier(Modifier::ITALIC),
            )));
            ListItem::new(lines).style(Style::default().fg(Color::Black).bg(Color::LightBlue))
        })
        .collect();

    let items = List::new(items)
        .block(Block::default().borders(Borders::ALL).title("List"))
        .highlight_style(
            Style::default()
                .bg(Color::LightGreen)
                .add_modifier(Modifier::BOLD),
        )
        .highlight_symbol(">> ");

    f.render_stateful_widget(items, chunks[0], &mut app.todos.state);
}
