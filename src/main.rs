mod app;
mod backend;
mod db;
mod ui;

use backend::run;
use db::init_db;
use std::error::Error;

use rusqlite::{Connection, Result};

fn main() -> Result<(), Box<dyn Error>> {
    let conn = Connection::open_in_memory()?;

    // for practice implementation, the init_db loads up a couple todos in memory on start
    let mut loaded_todos = init_db(&conn)?;
    let _ = run(&conn, loaded_todos.as_mut_slice());

    Ok(())
}
