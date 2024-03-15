use std::error::Error;

use mongodb::{ options::ClientOptions, Client};

pub async fn connect_to_db(db_name: String) -> Result<mongodb::Database, Box<dyn Error>> {
    let client_options = ClientOptions::parse("mongodb://localhost:27017")
        .await?;
    let client: Client = Client::with_options(client_options)?;

    let db: mongodb::Database = client.database(&db_name);

    Ok(db)
}
