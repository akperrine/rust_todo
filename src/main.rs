mod app;
mod backend;
mod db;
mod todo;
mod ui;

use backend::run;
use db::init_db;
use std::error::Error;

use rusqlite::Result;

fn main() -> Result<(), Box<dyn Error>> {
    let mut loaded_todos = init_db()?;
    let _ = run(loaded_todos.as_mut_slice());

    Ok(())
}
