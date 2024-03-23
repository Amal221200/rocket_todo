use serde::{Deserialize, Serialize};
use validator::Validate;
use todo_model::Todo;

#[derive(Validate, Deserialize, Serialize)]
pub struct NewTodoPayload {
    #[validate(length(min = 1, message = "pizza name is required"))]
    pub body: String,
    pub completed: bool
}

#[derive(Validate, Deserialize, Serialize)]
pub struct ReplacerPayload {
   pub replacer: Vec<Todo>
}

#[derive(Validate, Deserialize, Serialize)]
pub struct TodoParams {
    pub id: String,
}