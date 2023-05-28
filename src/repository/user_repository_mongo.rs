use std::env;
use std::error::Error;
use std::fmt::Debug;
use dotenv::dotenv;
use mongodb::{bson::doc, Client, Collection};
use mongodb::results::InsertOneResult;
use crate::models::User;
use crate::utils::generate_random_string;

pub struct UserRepository {
    collection: Collection<User>,
}

impl UserRepository {
    pub async fn init() -> Self {
        /*dotenv().ok();
        let uri = match env::var("MONGOURI") {
            Ok(v) => v.to_string(),
            Err(_) => format!("Error loading env variable"),
        };*/
        let uri = "mongodb://localhost:27017";
        let client = Client::with_uri_str(uri).await.unwrap();
        let db = client.database("rustDB");
        let col: Collection<User> = db.collection("User");
        UserRepository { collection: col }
    }

    pub async fn get_user(&self, id: &str) -> Option<User> {
        let filter = doc! { "id": id };
        self.collection.find_one(filter, None).await.unwrap()
    }

    pub async fn create_user(&self, new_user: User) -> Option<User> {
        let user = User {
            id: generate_random_string(20),
            name: new_user.name,
            surname: new_user.surname,
            city: new_user.city
        };

        match self.collection.insert_one(&user, None).await {
            Ok(_) => Some(user),
            Err(_) => None,
        }
    }
}