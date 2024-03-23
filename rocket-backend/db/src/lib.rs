use std::error::Error;
use wither::mongodb::{Client, Database};

pub async fn connect_to_db(db_url: &str, db_name: &str) -> Result<Database, Box<dyn Error>> {
    let db: Database = Client::with_uri_str(db_url)
        .await?
        .database(db_name);

    Ok(db)
}
