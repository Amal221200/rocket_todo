use actix_cors::Cors;
use actix_web::{
    http::header::{ACCEPT, AUTHORIZATION, CONTENT_TYPE},
    web::Data,
    App, HttpServer,
};
use db::connect_to_db;
use std::env;
use todo_controllers::{add_todo, delete_todo, get_todo, get_todos, update_order, update_todo};
use wither::mongodb::Database;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenvy::dotenv().expect("Failed to load env");

    let db: Database = connect_to_db(&env::var("DATABASE_NAME").expect("Failed to load db name")).await.expect("Failed to Connect");
    let db_data: Data<Database> = Data::new(db);
    
    HttpServer::new(move || {
        App::new()
            .wrap(
                Cors::default()
                    .allowed_origin(&env::var("ALLOWED_ORIGIN").expect("Failed to load db name"))
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
    })
    .on_connect(|_some, _ext| println!("Listening to http://localhost:8080, {:#?}", _ext))
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
