use crate::app::{App, InputMode};

use tui::{
    backend::Backend,
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Span, Spans, Text},
    widgets::{Block, Borders, List, ListItem, Paragraph},
    Frame,
};

use unicode_width::UnicodeWidthStr;

pub fn ui<B: Backend>(f: &mut Frame<B>, app: &mut App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(2)
        .constraints([Constraint::Percentage(85), Constraint::Length(2)].as_ref())
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
            ListItem::new(lines).style(Style::default().fg(Color::Black).bg(Color::White))
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

    let (msg, style) = match app.input_mode {
        InputMode::Normal => (
            vec![
                Span::raw("Press "),
                Span::styled("q", Style::default().add_modifier(Modifier::BOLD)),
                Span::raw(" to exit, "),
                Span::styled("a", Style::default().add_modifier(Modifier::BOLD)),
                Span::raw(" to add, "),
                Span::styled("TAB", Style::default().add_modifier(Modifier::BOLD)),
                Span::raw(" to toggle complete, "),
                Span::styled("e", Style::default().add_modifier(Modifier::BOLD)),
                Span::raw(" to edit selected, "),
                Span::styled("d", Style::default().add_modifier(Modifier::BOLD)),
                Span::raw(" to delete selected"),
            ],
            Style::default().add_modifier(Modifier::RAPID_BLINK),
        ),
        InputMode::EditingAdd | InputMode::EditingUpdate => (
            vec![
                Span::raw("Press "),
                Span::styled("Esc", Style::default().add_modifier(Modifier::BOLD)),
                Span::raw(" to stop editing, "),
                Span::styled("Enter", Style::default().add_modifier(Modifier::BOLD)),
                Span::raw(" to add todo"),
            ],
            Style::default(),
        ),
    };

    let mut text = Text::from(Spans::from(msg));
    text.patch_style(style);
    let help_message = Paragraph::new(text);
    f.render_widget(help_message, chunks[0]);

    let input = Paragraph::new(app.input.as_ref())
        .style(match app.input_mode {
            InputMode::Normal => Style::default(),
            InputMode::EditingAdd | InputMode::EditingUpdate => Style::default().fg(Color::Yellow),
        })
        .block(Block::default().borders(Borders::ALL).title("Input"));
    f.render_widget(input, chunks[1]);
    match app.input_mode {
        InputMode::Normal =>
            // Hide the cursor. `Frame` does this by default, so we don't need to do anything here
            {}

        InputMode::EditingAdd | InputMode::EditingUpdate => {
            // Make the cursor visible and ask tui-rs to put it at the specified coordinates after rendering
            f.set_cursor(
                // Put cursor past the end of the input text
                chunks[1].x + app.input.width() as u16 + 1,
                // Move one line down, from the border to the input line
                chunks[1].y + 1,
            )
        }
    }
}
