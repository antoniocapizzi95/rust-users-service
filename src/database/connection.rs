use std::env;
use dotenv::dotenv;
use mongodb::{Client};

pub async fn connection_mongo() -> mongodb::Database {
    dotenv().ok();
    let uri = match env::var("MONGOURI") {
        Ok(v) => v.to_string(),
        Err(_) => format!("Error loading env variable"),
    };
    let client = Client::with_uri_str(uri).await.unwrap();
    client.database("rustDB")
}