use std::{error::Error, env};
use wither::mongodb::{Client, Database};

pub async fn connect_to_db(db_name: &str) -> Result<Database, Box<dyn Error>> {

    let db:Database = Client::with_uri_str(&env::var("DATABASE_URL")?).await?.database(db_name);

    Ok(db)
}
