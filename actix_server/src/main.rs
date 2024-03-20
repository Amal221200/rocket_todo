use actix_cors::Cors;
use actix_web::{
    http::header::{ACCEPT, AUTHORIZATION, CONTENT_TYPE},
    web::Data,
    App, HttpServer,
};
use wither::mongodb::Database;
use todo_controllers::{add_todo, delete_todo, get_todos, update_order, update_todo, get_todo};
use db::connect_to_db;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let db: Database = connect_to_db(&"todoApp").await.expect("Failed to Connect");
    let db_data: Data<Database> = Data::new(db);

    HttpServer::new(move || {
        App::new()
            .wrap(
                Cors::default()
                    .allowed_origin("http://localhost:5173")
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
