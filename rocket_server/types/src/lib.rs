use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Todo {
    id: u32,
    body: String,
    completed: bool,
}

impl Todo {
    pub fn new(id: u32, body: String, completed: bool) -> Todo {
        Todo {
            id,
            body,
            completed,
        }
    }
}
