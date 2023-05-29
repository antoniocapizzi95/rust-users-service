use std::env;
use dotenv::dotenv;
use futures::StreamExt;
use mongodb::{bson::doc, Client, Collection};
use mongodb::results::{DeleteResult};
use crate::models::User;
use crate::utils::generate_random_string;
use anyhow::Result;

pub struct UserRepository {
    collection: Collection<User>,
}

impl UserRepository {
    pub async fn init() -> Self {
        dotenv().ok();
        let uri = match env::var("MONGOURI") {
            Ok(v) => v.to_string(),
            Err(_) => format!("Error loading env variable"),
        };
        let client = Client::with_uri_str(uri).await.unwrap();
        let db = client.database("rustDB");
        let col: Collection<User> = db.collection("User");
        UserRepository { collection: col }
    }

    pub async fn get_user(&self, user_id: &str) -> Option<User> {
        let filter = doc! { "id": user_id };
        self.collection.find_one(filter, None).await.unwrap()
    }

    pub async fn create_user(&self, new_user: User) -> Option<User> {
        let user = User {
            id: generate_random_string(10),
            name: new_user.name,
            surname: new_user.surname,
            city: new_user.city
        };

        match self.collection.insert_one(&user, None).await {
            Ok(_) => Some(user),
            Err(_) => None,
        }
    }

    pub async fn get_users(&self) -> Vec<User> {
        let mut cursor = self.collection.find(None, None).await.unwrap();
        let mut users = Vec::new();
        while let Some(user) = cursor.next().await {
            users.push(user.unwrap());
        }
        return users;
    }

    pub async fn update_user(&self, user_id: &str, new_user_data: User) -> Option<User> {
        let filter = doc! { "id": user_id };
        let update = doc! {
        "$set": {
            "name": new_user_data.name.clone(),
            "surname": new_user_data.surname.clone(),
            "city": new_user_data.city.clone()
            }
        };
        match self
            .collection
            .update_one(filter, update, None)
            .await
        {
            Ok(_) => Some(new_user_data),
            Err(_) => None,
        }
    }

    pub async fn delete_user(&self, user_id: &str) -> Result<DeleteResult, anyhow::Error> {
        let filter = doc! { "id": user_id };
        let result = self.collection.delete_one(filter, None).await?;
        Ok(result)
    }

}