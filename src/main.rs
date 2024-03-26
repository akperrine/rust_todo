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

// impl Repository for Connection {
//     fn add_todo(&self, todo: &Todo) -> Result<(), rusqlite::Error> {
//         let res = self
//             .execute(
//                 "INSERT INTO todo (message, complete) VALUES (?1, ?2)",
//                 (&todo.message, &todo.complete),
//             )
//             .map(|_| ());
//         println!("{:?}", res);
//         res
//     }

//     fn get_todos(&self) -> Result<Vec<Todo>, rusqlite::Error> {
//         let mut stmt = self.prepare("SELECT id, message, complete FROM todo")?;
//         let todos = stmt.query_map([], |row| {
//             Ok(Todo {
//                 id: row.get(0)?,
//                 message: row.get(1)?,
//                 complete: row.get(2)?,
//             })
//         })?;

//         todos.collect()
//     }

//     fn update_todo(&self, todo: &Todo) -> Result<(), rusqlite::Error> {
//         if let Some(id) = todo.id {
//             let mut stmt = self.prepare("SELECT id, message, complete FROM todo WHERE id = :id")?;
//             let res = stmt.query_map(&[(":id", &id)], |row| {
//                 Ok(Todo {
//                     id: row.get(0)?,
//                     message: row.get(1)?,
//                     complete: row.get(2)?,
//                 })
//             })?;

//             println!("HI");
//             let mut names = Vec::new();
//             for name_result in res {
//                 names.push(name_result?);
//             }
//             println!("{:?} hi", names);
//             Ok(())
//         } else {
//             println!("Cannot update todo with no ID");
//             Ok(())
//         }
//     }
// }

fn main() -> Result<(), Box<dyn Error>> {
    let conn = Connection::open_in_memory()?;

    let mut loaded_todos = init_db(&conn)?;
    let _ = run(&conn, loaded_todos.as_mut_slice());
    let todo = Todo {
        id: Some(1),
        message: String::from("checking"),
        complete: 0,
    };
    // conn.update_todo(&todo).unwrap();

    Ok(())
}
