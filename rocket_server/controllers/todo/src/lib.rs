#[macro_use]
extern crate rocket;
use mongo::connect_to_db;
use mongodb::bson::{doc, Document, Serializer};
use rocket::{
    futures::TryStreamExt, serde::{json::Json, Serialize}
};
use types::Todo;

#[get("/")]
pub async fn get_todos() -> Result<Json<Vec<Todo>>, String> {
    let db = connect_to_db("todoApp".to_string())
        .await
        .map_err(|err| err.to_string())?;

    let todos_cursor: mongodb::Cursor<Todo> = db
        .collection("todos")
        .find(None, None)
        .await
        .map_err(|err| err.to_string())?;

    let result: Vec<Todo> = todos_cursor.try_collect().await.expect("Failed to convert document.");

    Ok(Json(result))
}

#[get("/<id>")]
pub async fn get_todo(id: u32) -> Result<Json<Todo>, String> {
    let db = connect_to_db("todoApp".to_string())
        .await
        .map_err(|err| err.to_string())?;

    let filter = doc! {"id": id};

    let todo_cursor:  Option<Todo> = db
        .collection("todos")
        .find_one(filter, None)
        .await.map_err(|err| err.to_string())?;
    
    
    let result: Todo = todo_cursor.unwrap();

    Ok(Json(result))
}

#[post("/", format = "application/json", data = "<todo>")]
pub async fn add_todo(todo: Json<Todo>) -> Result<Json<mongodb::results::InsertOneResult>, String> {
    let db = connect_to_db("todoApp".to_string())
        .await
        .map_err(|err| err.to_string())?;

    let serialized_todo = todo
        .serialize(Serializer::new())
        .map_err(|err| err.to_string())?;

    let new_todo_doc = Document::from(serialized_todo.as_document().unwrap().clone());

    let inserted_result: mongodb::results::InsertOneResult = db
        .collection("todos")
        .insert_one(new_todo_doc, None)
        .await
        .map_err(|err| err.to_string())?;

    Ok(Json(inserted_result))
}

#[put("/<id>", format = "application/json", data = "<todo>")]
pub async fn update_todo(id: u32, todo: Json<Todo>) -> Result<Json<u64>, String> {
    let db = connect_to_db("todoApp".to_string())
        .await
        .map_err(|err| err.to_string())?;

    let serialized_todo = todo
        .serialize(Serializer::new())
        .map_err(|err| err.to_string())?;

    let query = doc! {"id": id};
    let update = doc! {"$set": Document::from(serialized_todo.as_document().unwrap().clone())};

    let updated_result: mongodb::results::UpdateResult = db
        .collection::<Todo>("todos")
        .update_one(query, update, None)
        .await
        .map_err(|err| err.to_string())?;

    Ok(Json(updated_result.modified_count))
}

#[delete("/<id>")]
pub async fn delete_todo(id: u32) -> Result<Json<u64>, String> {
    let db = connect_to_db("todoApp".to_string())
        .await
        .map_err(|err| err.to_string())?;

    let query = doc! {"id": id};

    let deleted_result: mongodb::results::DeleteResult = db
        .collection::<Todo>("todos")
        .delete_one(query, None)
        .await
        .map_err(|err| err.to_string())?;

    Ok(Json(deleted_result.deleted_count))
}
