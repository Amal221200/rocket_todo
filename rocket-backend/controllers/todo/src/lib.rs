use std::str::FromStr;

use actix_web::{
    delete, get, patch, post, put,
    web::{Data, Json, Path},
};
use futures::TryStreamExt;
use todo_model::Todo;
use utils_errors::TodoError;
use utils_types::{NewTodoPayload, ReplacerPayload, TodoParams};
use validator::Validate;
use wither::{
    bson::{doc, oid::ObjectId},
    mongodb::Database,
    Model,
};

#[get("")]
async fn get_todos(db: Data<Database>) -> Result<Json<Vec<Todo>>, TodoError> {
    Todo::sync(&db)
        .await
        .map_err(|_| TodoError::InternalServerError)?;

    let result = Todo::find(&db, None, None).await;

    match result {
        Ok(cursor) => {
            let todos: Vec<Todo> = cursor.try_collect().await.expect("Failed to convert");
            Ok(Json(todos))
        }
        Err(_) => Err(TodoError::NoTodoFound),
    }
}

#[get("/{id}")]
async fn get_todo(params: Path<TodoParams>, db: Data<Database>) -> Result<Json<Todo>, TodoError> {
    Todo::sync(&db)
        .await
        .map_err(|_| TodoError::InternalServerError)?;

    let id = params.into_inner().id;
    let oid = ObjectId::from_str(&id)
        .map_err(|err| err.to_string())
        .expect("msg");
    let filter_query = doc! {"_id": oid};
    let todo_exists: Result<Option<Todo>, wither::WitherError> =
        Todo::find_one(&db, filter_query.clone(), None).await;

    match todo_exists {
        Ok(result) => match result {
            Some(todo) => Ok(Json(todo)),
            None => Err(TodoError::InternalServerError),
        },
        Err(_) => Err(TodoError::NoSuchTodoFound),
    }
}

#[post("")]
async fn add_todo(body: Json<NewTodoPayload>, db: Data<Database>) -> Result<Json<Todo>, TodoError> {
    match body.validate() {
        Ok(_) => {
            Todo::sync(&db)
                .await
                .map_err(|_| TodoError::InternalServerError)?;
            let todo_payload = body.into_inner();

            let mut new_todo = Todo {
                id: None,
                body: todo_payload.body,
                completed: todo_payload.completed,
            };

            match new_todo.save(&db, None).await {
                Ok(_) => Ok(Json(new_todo)),
                Err(_) => Err(TodoError::InternalServerError),
            }
        }
        Err(_) => Err(TodoError::InvalidData),
    }
}

#[put("/{id}")]
async fn update_todo(
    params: Path<TodoParams>,
    body: Json<Todo>,
    db: Data<Database>,
) -> Result<Json<Todo>, TodoError> {
    match body.validate() {
        Ok(_) => {
            Todo::sync(&db)
                .await
                .map_err(|_| TodoError::InternalServerError)?;
            let id = params.into_inner().id;
            let oid = ObjectId::from_str(&id)
                .map_err(|err| err.to_string())
                .expect("msg");
            let filter_query = doc! {"_id": oid};
            let todo_exists = Todo::find_one(&db, filter_query.clone(), None).await;

            match todo_exists {
                Ok(res) => {
                    let payload = body.into_inner();
                    let mut todo = res.unwrap();
                    todo.body = payload.body;
                    todo.completed = payload.completed;

                    match todo.save(&db, None).await {
                        Ok(_) => Ok(Json(todo)),
                        Err(_) => Err(TodoError::InternalServerError),
                    }
                }
                Err(_) => Err(TodoError::NoSuchTodoFound),
            }
        }
        Err(_) => Err(TodoError::InvalidData),
    }
}

#[delete("/{id}")]
async fn delete_todo(
    params: Path<TodoParams>,
    db: Data<Database>,
) -> Result<Json<Todo>, TodoError> {
    Todo::sync(&db)
        .await
        .map_err(|_| TodoError::InternalServerError)?;
    let id = params.into_inner().id;
    let oid = ObjectId::from_str(&id)
        .map_err(|err| err.to_string())
        .expect("msg");
    let filter_query = doc! {"_id": oid};
    let todo_exists: Result<Option<Todo>, wither::WitherError> =
        Todo::find_one(&db, filter_query.clone(), None).await;

    match todo_exists {
        Ok(todo) => match todo {
            Some(deleted_todo) => {
                deleted_todo
                    .delete(&db)
                    .await
                    .map_err(|_| TodoError::InternalServerError)?;
                Ok(Json(deleted_todo))
            }
            None => Err(TodoError::InternalServerError),
        },
        Err(_) => Err(TodoError::NoSuchTodoFound),
    }
}

#[patch("")]
async fn update_order(
    db: Data<Database>,
    body: Json<ReplacerPayload>,
) -> Result<Json<bool>, TodoError> {
    match body.validate() {
        Ok(_) => {
            Todo::sync(&db)
                .await
                .map_err(|_| TodoError::InternalServerError)?;

            let new_todos = body.into_inner().replacer;

            let _ = Todo::delete_many(&db, doc! {}, None)
                .await
                .map_err(|_| TodoError::InternalServerError);

            let new_todos_docs = new_todos.iter().map(|todo| {
                match todo
                    .document_from_instance()
                    .map_err(|_| TodoError::InternalServerError)
                {
                    Ok(res) => Some(res),
                    Err(_) => None,
                }
                .unwrap()
            });

            let _ = Todo::collection(&db)
                .insert_many(new_todos_docs, None)
                .await
                .map_err(|_| TodoError::InternalServerError);
            Ok(Json(true))
        }
        Err(_) => Err(TodoError::InvalidData),
    }
}
