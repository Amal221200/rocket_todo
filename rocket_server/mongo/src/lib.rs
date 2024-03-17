use std::error::Error;
use wither::mongodb::{Client, Database};

pub async fn connect_to_db(db_name: &str) -> Result<Database, Box<dyn Error>> {

    let db:Database = Client::with_uri_str("mongodb+srv://Amal_admin:eqUOI4QEAaNoHiyl@amalcluster.btbwg.mongodb.net/todoApp?retryWrites=true&w=majority").await?.database(db_name);

    Ok(db)
}
