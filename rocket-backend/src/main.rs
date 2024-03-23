use actix_cors::Cors;
use actix_web::{
    get,
    http::header::{ACCEPT, AUTHORIZATION, CONTENT_TYPE},
    web::{self, Data, ServiceConfig}
};
use db::connect_to_db;
use shuttle_actix_web::ShuttleActixWeb;
use std::env;
use todo_controllers::{add_todo, delete_todo, get_todo, get_todos, update_order, update_todo};
use wither::mongodb::Database;

#[get("/")]
async fn hello_world() -> &'static str {
    "Hello World!"
}

#[shuttle_runtime::main]
async fn main() -> ShuttleActixWeb<impl FnOnce(&mut ServiceConfig) + Send + Clone + 'static> {
    dotenvy::dotenv().expect("Failed to load env");
    
    let db: Database = connect_to_db(&env::var("DATABASE_NAME").expect("Failed to load db name"))
        .await
        .expect("Failed to Connect");

    let db_data: Data<Database> = Data::new(db);
    let config = move |cfg: &mut ServiceConfig| {
        cfg.service(
            web::scope("/todo")
                .wrap(
                    Cors::default()
                        .allowed_origin(
                            &env::var("ALLOWED_ORIGIN").expect("Failed to load db name"),
                        )
                        .allowed_methods(vec!["GET", "POST", "PATCH", "DELETE", "PUT"])
                        .allowed_headers(vec![CONTENT_TYPE, AUTHORIZATION, ACCEPT])
                        .supports_credentials()
                        .max_age(3600),
                )
                .app_data(db_data.clone())
                .service(get_todos)
                .service(get_todo)
                .service(add_todo)
                .service(update_todo)
                .service(update_order)
                .service(delete_todo)
        );
    };

    Ok(config.into())
}
