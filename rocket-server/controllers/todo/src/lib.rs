#[macro_use]
extern crate rocket;
use std::str::FromStr;

use rocket::{
    futures::{TryFutureExt, TryStreamExt},
    serde::json::Json,
    State,
};
use types::Todo;
use wither::{
    bson::{doc, oid::ObjectId},
    mongodb::{options::FindOptions, Database},
    Model,
};

#[get("/")]
pub async fn get_todos(db: &State<Database>) -> Result<Json<Vec<Todo>>, String> {
    Todo::sync(&db).map_err(|err| err.to_string()).await?;

    let _option_query = FindOptions::builder().sort(doc! {"order": 1}).build();

    let cursor = Todo::find(&db, None, None)
        .map_err(|err| err.to_string())
        .await?;

    let result: Vec<Todo> = cursor
        .try_collect()
        .await
        .expect("Failed to convert document.");
    Ok(Json(result))
}

#[get("/<id>")]
pub async fn get_todo(db: &State<Database>, id: &str) -> Result<Json<Todo>, String> {
    Todo::sync(&db).map_err(|err| err.to_string()).await?;

    let oid = ObjectId::from_str(&id).map_err(|err| err.to_string())?;

    let filter = doc! {"_id": oid};

    let result = Todo::find_one(&db, filter, None)
        .map_err(|err| err.to_string())
        .await?;

    let todo: Todo = result.unwrap();
    println!("{:#?}", &todo.id);
    Ok(Json(todo))
}

#[post("/", format = "application/json", data = "<todo>")]
pub async fn add_todo(db: &State<Database>, todo: Json<Todo>) -> Result<Json<Todo>, String> {
    Todo::sync(&db).map_err(|err| err.to_string()).await?;

    let deserialized_todo: Todo = todo.into_inner();
    // let _total_todos = (get_todos(&db).map_err(|err| err.to_string()).await?).len();

    let mut new_todo_doc = Todo {
        id: None,
        body: deserialized_todo.body,
        completed: deserialized_todo.completed
    };

    new_todo_doc
        .save(&db, None)
        .map_err(|err| err.to_string())
        .await?;

    Ok(Json(new_todo_doc))
}

#[put("/<id>", format = "application/json", data = "<todo>")]
pub async fn update_todo(
    db: &State<Database>,
    id: &str,
    todo: Json<Todo>,
) -> Result<Json<Todo>, String> {
    Todo::sync(&db).map_err(|err| err.to_string()).await?;
    let oid = ObjectId::from_str(&id).map_err(|err| err.to_string())?;

    let deserialized_todo: Todo = todo.into_inner();
    let filter_query = doc! {"_id": oid};

    let result = Todo::find_one(&db, filter_query, None)
        .map_err(|err| err.to_string())
        .await?;

    let mut todo = result.unwrap();

    todo.body = deserialized_todo.body;
    todo.completed = deserialized_todo.completed;
    todo.save(&db, None).map_err(|err| err.to_string()).await?;

    Ok(Json(todo))
}

#[patch("/", format = "application/json", data = "<replacer>")]
pub async fn update_order(
    db: &State<Database>,
    replacer: Json<Vec<Todo>>,
) -> Result<Json<bool>, String> {
    Todo::sync(&db).map_err(|err| err.to_string()).await?;

    let new_todos = replacer.into_inner();

    Todo::delete_many(&db, doc! {}, None)
        .map_err(|err| err.to_string())
        .await?;

    let new_todos_docs = new_todos.iter().map(|todo| {
        match todo.document_from_instance().map_err(|err| err.to_string()) {
            Ok(res) => Some(res),
            Err(_) => None,
        }.unwrap()
    });

    Todo::collection(&db)
        .insert_many(new_todos_docs, None)
        .map_err(|err| err.to_string())
        .await?;
    
    Ok(Json(true))
}

#[delete("/<id>")]
pub async fn delete_todo(db: &State<Database>, id: &str) -> Result<Json<Todo>, String> {
    Todo::sync(&db).map_err(|err| err.to_string()).await?;

    let oid = ObjectId::from_str(&id).map_err(|err| err.to_string())?;
    let filter_query = doc! {"_id": oid};

    let result = Todo::find_one(&db, filter_query, None)
        .map_err(|err| err.to_string())
        .await?;

    let todo = result.unwrap();

    todo.delete(&db).map_err(|err| err.to_string()).await?;

    Ok(Json(todo))
}
