#[macro_use]
extern crate rocket;
#[macro_use]
extern crate dotenv_codegen;

use mongo::connect_to_db;
use rocket::{futures::TryFutureExt, Build, Rocket};
use todo;
use wither::mongodb::Database;
mod cors;

#[get("/")]
fn index() -> String {
    format!("Hello {name}", name = "Rocket")
}

#[launch]
async fn launch() -> Rocket<Build> {
    let db: Option<Database> = match connect_to_db(dotenv!("DATABASE_NAME"))
        .map_err(|err| err.to_string())
        .await
    {
        Ok(db) => Some(db),
        Err(_err) => None,
    };

    rocket::build()
    .mount("/", routes![index])
        .attach(cors::CORS)
        .manage(db.unwrap())
        .mount(
            "/todo",
            routes![
                todo::get_todos,
                todo::add_todo,
                todo::get_todo,
                todo::update_todo,
                todo::delete_todo
            ],
        )
}
