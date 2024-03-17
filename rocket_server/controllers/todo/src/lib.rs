#[macro_use]
extern crate rocket;
use std::str::FromStr;

use rocket::{
    futures::{TryFutureExt, TryStreamExt},
    serde::json::Json,
    State,
};
use wither::{
    bson::{doc, oid::ObjectId},
    mongodb::{options::FindOptions, Database},
    Model,
};
use types::{Todo, Replacer};

#[get("/")]
pub async fn get_todos(db: &State<Database>) -> Result<Json<Vec<Todo>>, String> {
    Todo::sync(&db).map_err(|err| err.to_string()).await?;

    let _option_query = FindOptions::builder().sort(doc! {"order": 1}).build();
    
    let cursor = Todo::find(&db, None, _option_query)
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
    let total_todos = (get_todos(&db).map_err(|err| err.to_string()).await?).len();

    let mut new_todo_doc = Todo {
        id: None,
        body: deserialized_todo.body,
        completed: deserialized_todo.completed,
        order: (total_todos as i32) + 1,
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


#[patch("/<id>", format = "application/json", data = "<replacer>")]
pub async fn update_order(
    db: &State<Database>,
    id: String,
    replacer: Json<Replacer>,
) -> Result<Json<bool>, String> {
    Todo::sync(&db).map_err(|err| err.to_string()).await?;

    let oid_1 = ObjectId::from_str(&id).map_err(|err| err.to_string())?;
    let filter_query_1 = doc! {"_id": oid_1};

    let oid_2 = ObjectId::from_str(&replacer.into_inner().replace_id).map_err(|err| err.to_string())?;
    let filter_query_2 = doc! {"_id": oid_2};

    let result_1 = Todo::find_one(&db, filter_query_1, None)
        .map_err(|err| err.to_string())
        .await?;
    let result_2 = Todo::find_one(&db, filter_query_2, None)
        .map_err(|err| err.to_string())
        .await?;

    let mut main_todo = result_1.unwrap();
    let mut replace_todo = result_2.unwrap();

    let ord_1 = main_todo.order;
    let ord_2 = replace_todo.order;

    main_todo.order = ord_2;
    replace_todo.order = ord_1;

    main_todo
        .save(&db, None)
        .map_err(|err| err.to_string())
        .await?;
    replace_todo
        .save(&db, None)
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
