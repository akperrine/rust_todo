use std::error::Error;

use rusqlite::Connection;

use crate::todo::Todo;

pub trait Repository {
    fn add_todo(&self, todo: &Todo) -> Result<(), rusqlite::Error>;
    fn get_todos(&self) -> Result<Vec<Todo>, rusqlite::Error>;
    fn update_todo(&self, todo: &Todo) -> Result<(), rusqlite::Error>;
    fn delete_todo(&self, todo: u32) -> Result<(), rusqlite::Error>;
}

pub fn init_db(conn: &Connection) -> Result<Vec<Todo>, Box<dyn Error>> {
    // SQLite Setup

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

    Ok(loaded_todos)
}

// pub fn insert_todo(conn: &Connection, todo: Todo) -> Result<(), rusqlite::Error> {
//     let sql = "INSERT INTO todo (message, complete) VALUES (?, ?)";

//     conn.execute(sql, (todo.message, todo.complete))?;

//     Ok(())
// }
