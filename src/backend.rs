use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};

use tui::{
    backend::{Backend, CrosstermBackend},
    widgets::ListState,
    Terminal,
};

use rusqlite::Connection;

use crate::ui::ui;
use crate::{app::App, db::Repository};
use crate::{app::InputMode, todo::Todo};
use std::{error::Error, io};

fn run_app<B: Backend>(
    conn: &Connection,
    terminal: &mut Terminal<B>,
    mut app: App,
) -> io::Result<()> {
    loop {
        terminal.draw(|f| ui(f, &mut app))?;

        if let Event::Key(key) = event::read()? {
            match app.input_mode {
                InputMode::Normal => match key.code {
                    KeyCode::Char('q') => return Ok(()),
                    KeyCode::Char('a') => {
                        app.input_mode = InputMode::EditingAdd;
                    }
                    KeyCode::Char('e') => {
                        app.input_mode = InputMode::EditingUpdate;
                    }
                    KeyCode::Left => app.todos.unselect(),
                    KeyCode::Down => app.todos.next(),
                    KeyCode::Up => app.todos.previous(),
                    _ => {}
                },
                InputMode::EditingAdd | InputMode::EditingUpdate => match key.code {
                    KeyCode::Enter => {
                        if app.input != "" {
                            match app.input_mode {
                                InputMode::EditingAdd => {
                                    let todo = Todo {
                                        id: None,
                                        message: app.input.drain(..).collect(),
                                        complete: 1,
                                    };
                                    app.add_todo(&todo).unwrap();
                                }
                                InputMode::EditingUpdate => {
                                    let selected_index = app.todos.state.selected().unwrap();
                                    // println!("{:?} hi", selected_index);
                                    let selected_todo =
                                        app.todos.items.get(selected_index).unwrap();
                                    // println!("{:?} hii", selected_todo);
                                    let todo = Todo {
                                        id: selected_todo.id,
                                        message: app.input.drain(..).collect(),
                                        complete: selected_todo.complete,
                                    };
                                    // println!("{:?}", todo);
                                    app.update_todo(&todo);
                                }
                                _ => {}
                            }

                            let todos = app.get_todos().unwrap();
                            println!("{:?} todods", todos);

                            match app.input_mode {
                                InputMode::EditingUpdate | InputMode::EditingAdd => {
                                    println!("refresh");
                                    app.todos.refresh_items(&todos);
                                }
                                _ => {}
                            }
                        }
                    }
                    KeyCode::Char(x) => {
                        app.input.push(x);
                    }
                    KeyCode::Backspace => {
                        app.input.pop();
                    }
                    KeyCode::Esc => {
                        app.input_mode = InputMode::Normal;
                    }
                    _ => {}
                },
            }
        }
    }
}

pub fn run(conn: &Connection, starting_todos: &[Todo]) -> Result<(), Box<dyn Error>> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // app create and run
    let app = App::new(&starting_todos, conn);
    let res = run_app(&conn, &mut terminal, app);
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
