#[derive(Clone, Debug)]
pub struct Todo {
    pub id: Option<u32>,
    pub message: String,
    pub complete: u8,
}

impl Todo {
    // pub fn add(message: String) -> Result<Todo, rusqlite::Error> {
    //     let new_todo = Todo {
    //         id: None,
    //         message: message,
    //         complete: 1,
    //     };
    //     // insert_todo(conn, todo);
    //     Ok(new_todo)
    // }

    // pub fn edit(todo: Todo) -> Todo {
    //     return Todo;
    // }

    // pub fn delete(id: u32) -> () {}
}
