mod todo;
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::{error::Error, io};
use todo::Todo;

use tui::{
    backend::{Backend, CrosstermBackend},
    layout::{Constraint, Corner, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Span, Spans},
    widgets::{Block, Borders, List, ListItem, ListState},
    Frame, Terminal,
};

use rusqlite::{Connection, Result};

struct StatefulList<T> {
    items: Vec<T>,
    state: ListState,
}

impl<T> StatefulList<T> {
    fn with_items(items: Vec<T>) -> StatefulList<T> {
        StatefulList {
            state: ListState::default(),
            items,
        }
    }

    fn next(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i >= self.items.len() {
                    0
                } else {
                    i + 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i))
    }

    fn previous(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i == 0 {
                    self.items.len() - 1
                } else {
                    i - 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
    }

    fn unselect(&mut self) {
        self.state.select(None);
    }
}

struct App {
    todos: StatefulList<Todo>,
}

impl App {
    fn new(found_todos: &[Todo]) -> App {
        App {
            todos: StatefulList::with_items(found_todos.to_vec()),
        }
    }
}

fn run_app<B: Backend>(terminal: &mut Terminal<B>, mut app: App) -> io::Result<()> {
    loop {
        terminal.draw(|f| ui(f, &mut app))?;

        // if crossterm::event::poll(timeout)? {
        if let Event::Key(key) = event::read()? {
            match key.code {
                KeyCode::Char('q') => return Ok(()),
                KeyCode::Left => app.todos.unselect(),
                KeyCode::Down => app.todos.next(),
                KeyCode::Up => app.todos.previous(),
                _ => {}
            }
            // }
        }
    }
}

fn ui<B: Backend>(f: &mut Frame<B>, app: &mut App) {
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

fn main() -> Result<(), Box<dyn Error>> {
    // SQLite Setup
    let conn = Connection::open_in_memory()?;

    conn.execute(
        "CREATE TABLE todo (
            id INTEGER PRIMARY KEY NOT NULL,
            message TEXT NOT NULL,
            complete INTEGER
    )",
        (),
    )?;

    let sql = "INSERT INTO todo (message, complete) VALUES (?, ?)";

    let todo_to_add = Todo {
        id: None,
        message: String::from("First to do"),
        complete: 1,
    };

    let todo_to_add_2 = Todo {
        id: None,
        message: String::from("Second to do"),
        complete: 1,
    };
    // TODO set up Tui-rs
    // TODO then start to add in the todo aspect;
    conn.execute(sql, (&todo_to_add.message, &todo_to_add.complete))?;
    conn.execute(sql, (&todo_to_add_2.message, &todo_to_add_2.complete))?;

    let mut stmt = conn.prepare("SELECT id, message, complete FROM todo")?;
    let todo_iter = stmt.query_map([], |row| {
        Ok(Todo {
            id: row.get(0)?,
            message: row.get(1)?,
            complete: row.get(2)?,
        })
    })?;

    let mut loaded_todos: Vec<Todo> = vec![];

    for todo in todo_iter {
        println!("Found todo {:?}", todo.as_ref());
        loaded_todos.push(todo.unwrap());
    }
    // ***TUI setup
    // terminal setup
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // app create and run
    let app = App::new(&loaded_todos.as_mut_slice());
    let res = run_app(&mut terminal, app);
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    if let Err(err) = res {
        println!("{:?}", err)
    }

    Ok(())
}
