use crate::models::user_model::User;
use async_trait::async_trait;

#[async_trait]
pub trait UserRepository: Send + Sync {
    async fn get_user(&self, user_id: &str) -> Option<User>;
    async fn create_user(&self, new_user: User) -> Option<User>;
    async fn get_users(&self) -> Vec<User>;
    async fn update_user(&self, user_id: &str, new_user_data: User) -> Option<User>;
    async fn delete_user(&self, user_id: &str) -> Result<bool, anyhow::Error>;
}