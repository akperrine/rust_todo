mod app;
mod backend;
mod db;
mod todo;
mod ui;

use backend::run;
use db::{init_db, Repository};
use std::error::Error;
use todo::Todo;

use rusqlite::{Connection, Result};

//TODOs
// 1.) Remove using of connection from app
// 2.) implement back in the add and get method with the App
// 3.) implement this back with edit
// 4.) implement with Delete
fn main() -> Result<(), Box<dyn Error>> {
    let conn = Connection::open_in_memory()?;

    let mut loaded_todos = init_db(&conn)?;
    let _ = run(&conn, loaded_todos.as_mut_slice());
    // let todo = Todo {
    //     id: Some(1),
    //     message: String::from("checking"),
    //     complete: 0,
    // };
    // conn.update_todo(&todo).unwrap();

    Ok(())
}
