use actix_cors::Cors;
use actix_web::{
    http::header::{ACCEPT, AUTHORIZATION, CONTENT_TYPE},
    web::{self, Data, ServiceConfig},
};
use db::connect_to_db;
use shuttle_actix_web::ShuttleActixWeb;
use shuttle_runtime::SecretStore;
use todo_controllers::{add_todo, delete_todo, get_todo, get_todos, update_order, update_todo};
use wither::mongodb::Database;


#[shuttle_runtime::main]
async fn main(
    #[shuttle_runtime::Secrets] secrets: SecretStore,
) -> ShuttleActixWeb<impl FnOnce(&mut ServiceConfig) + Send + Clone + 'static> {

    let db: Database = connect_to_db(&secrets.get("DATABASE_URL").expect("Failed to get url"), &secrets.get("DATABASE_NAME").expect("Failed to get name"))
        .await
        .expect("Failed to Connect");

    let db_data: Data<Database> = Data::new(db);

    let config = move |cfg: &mut ServiceConfig| {
        cfg.service(
            web::scope("/todo")
                .wrap(
                    Cors::default()
                        .allowed_origin(
                            &secrets.get("ALLOWED_ORIGIN").expect("Failed to load db name"),
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
