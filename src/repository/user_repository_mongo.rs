use futures::StreamExt;
use mongodb::{bson::doc, Client, Collection};
use crate::models::user_model::User;
use anyhow::Result;
use crate::repository::user_repository::UserRepository;
use async_trait::async_trait;

pub struct UserRepositoryMongo {
    pub collection: Collection<User>,
}

#[async_trait]
impl UserRepository for UserRepositoryMongo {

    async fn get_user(&self, user_id: &str) -> Option<User> {
        let filter = doc! { "id": user_id };
        self.collection.find_one(filter, None).await.unwrap()
    }

    async fn create_user(&self, new_user: User) -> Option<User> {
        let user = User {
            id: new_user.id,
            name: new_user.name,
            surname: new_user.surname,
            city: new_user.city
        };

        match self.collection.insert_one(&user, None).await {
            Ok(_) => Some(user),
            Err(_) => None,
        }
    }

    async fn get_users(&self) -> Vec<User> {
        let mut cursor = self.collection.find(None, None).await.unwrap();
        let mut users = Vec::new();
        while let Some(user) = cursor.next().await {
            users.push(user.unwrap());
        }
        return users;
    }

    async fn update_user(&self, user_id: &str, new_user_data: User) -> Option<User> {
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

    async fn delete_user(&self, user_id: &str) -> Result<bool, anyhow::Error> {
        let filter = doc! { "id": user_id };
        let result = self.collection.delete_one(filter, None).await?;
        Ok(result.deleted_count > 0)
    }

}