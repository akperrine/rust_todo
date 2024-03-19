#[derive(Clone, Debug)]
pub struct Todo {
    pub id: Option<u32>,
    pub message: String,
    pub complete: u8,
}

impl<'a> Todo {
    fn add(message: String) -> Todo {
        return Todo {
            id: None,
            message: message,
            complete: 1,
        };
    }

    // fn edit(todo: Todo) -> Todo {}

    // fn delete(id: Uuid) -> () {}
}
