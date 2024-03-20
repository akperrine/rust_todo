mod app;
mod backend;
mod todo;
mod ui;

use backend::run;
use std::{error::Error, io};
use todo::Todo;

use rusqlite::{Connection, Result};

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
    //
    let _ = run(loaded_todos.as_mut_slice());

    Ok(())
}
