use crate::{db::Repository, todo::Todo};
use rusqlite::{Connection, OptionalExtension};
use tui::widgets::ListState;
pub struct StatefulList<T>
where
    T: Clone,
{
    pub items: Vec<T>,
    pub state: ListState,
}

impl<T> StatefulList<T>
where
    T: Clone,
{
    fn with_items(items: Vec<T>) -> StatefulList<T> {
        StatefulList {
            state: ListState::default(),
            items,
        }
    }

    pub fn next(&mut self) {
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

    pub fn previous(&mut self) {
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

    pub fn unselect(&mut self) {
        self.state.select(None);
    }

    pub fn add(&mut self, item: &T) {
        self.items.push(item.clone());
    }

    pub fn update(&mut self, item: &T) {
        // self.items.iter().find(|x| x.id == todo.item);
    }

    pub fn refresh_items(&mut self, items: &[T]) {
        self.items = items.to_vec();
    }

    // pub fn delete(&mut self, todo: &T) {}
}

pub enum InputMode {
    Normal,
    EditingAdd,
    EditingUpdate,
}

pub struct App<'a> {
    pub todos: StatefulList<Todo>,
    pub input: String,
    pub input_mode: InputMode,
    pub connection: &'a Connection,
}

impl<'a> App<'a> {
    pub fn new(found_todos: &[Todo], connection: &'a Connection) -> Self {
        App {
            todos: StatefulList::with_items(found_todos.to_vec()),
            input: String::new(),
            input_mode: InputMode::Normal,
            connection,
        }
    }
}

impl<'a> Repository for App<'a> {
    fn add_todo(&self, todo: &Todo) -> Result<(), rusqlite::Error> {
        let res = self
            .connection
            .execute(
                "INSERT INTO todo (message, complete) VALUES (?1, ?2)",
                (&todo.message, &todo.complete),
            )
            .map(|_| ());
        res
    }

    fn get_todos(&self) -> Result<Vec<Todo>, rusqlite::Error> {
        let mut stmt = self
            .connection
            .prepare("SELECT id, message, complete FROM todo")?;
        let todos = stmt.query_map([], |row| {
            Ok(Todo {
                id: row.get(0)?,
                message: row.get(1)?,
                complete: row.get(2)?,
            })
        })?;

        todos.collect()
    }

    fn update_todo(&self, todo: &Todo) -> Result<(), rusqlite::Error> {
        let todo_exists = self
            .connection
            .query_row("SELECT 1 FROM todo WHERE id = ?", &[&todo.id], |_| Ok(true))
            .unwrap_or(false);

        if todo_exists {
            self.connection.execute(
                "UPDATE todo SET message = ?, complete = ? WHERE id = ?",
                &[
                    &todo.message,
                    &todo.complete.to_string(),
                    &todo.id.unwrap().to_string(),
                ],
            )?;
            Ok(())
        } else {
            Err(rusqlite::Error::QueryReturnedNoRows)
        }
    }
}
