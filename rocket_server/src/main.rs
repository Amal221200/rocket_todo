#[macro_use]
extern crate rocket;
#[macro_use]
extern crate dotenv_codegen;

use rocket::{futures::TryFutureExt, http::Method, Build, Rocket};
use rocket_cors::{AllowedHeaders, AllowedOrigins};
use wither::mongodb::Database;

use todo;
use mongo::connect_to_db;

#[get("/")]
fn index() -> String {
    format!("Hello {name}", name = "Rocket")
}

#[launch]
async fn launch() -> Rocket<Build> {
    
    let allowed_origins: rocket_cors::AllOrSome<rocket_cors::Origins> =
        AllowedOrigins::some_exact(&[dotenv!("ALLOWED_ORIGIN")]);

    let db: Option<Database> = match connect_to_db(dotenv!("DATABASE_NAME"))
        .map_err(|err| err.to_string())
        .await
    {
        Ok(db) => Some(db),
        Err(_err) => None,
    };

    let cors_options: Result<rocket_cors::Cors, String> = rocket_cors::CorsOptions {
        allowed_origins,
        allowed_methods: vec![Method::Get, Method::Post, Method::Put, Method::Delete]
            .into_iter()
            .map(From::from)
            .collect(),
        allowed_headers: AllowedHeaders::some(&["Authorization", "Accept"]),
        allow_credentials: true,
        ..Default::default()
    }
    .to_cors()
    .map_err(|err: rocket_cors::Error| err.to_string());

    let cors: Option<rocket_cors::Cors> = match cors_options {
        Ok(cors) => Some(cors),
        Err(_err) => {
            println!("{}", _err);
            None
        }
    };

    rocket::build()
        .mount("/", routes![index])
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
        .attach(cors.unwrap())
}
