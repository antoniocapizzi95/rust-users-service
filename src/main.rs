mod models;
mod database;
mod repository;
mod utils;
mod api;

use actix_web::{App, HttpServer};
use actix_web::web::Data;
use crate::repository::user_repository_mongo::UserRepositoryMongo;

use crate::api::user_routes::{create_user, delete_user, get_user, get_users, update_user};
use crate::database::connection::connection_mongo;
use crate::repository::user_repository::UserRepository;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let col = connection_mongo().await.collection("User");
    let repo_impl = UserRepositoryMongo{collection: col};
    let repo: Box<dyn UserRepository> = Box::new(repo_impl);
    let repo_data = Data::new(repo);

    HttpServer::new(move || {
        App::new()
            .app_data(repo_data.clone())
            .service(get_user)
            .service(get_users)
            .service(create_user)
            .service(update_user)
            .service(delete_user)
    })
        .bind("127.0.0.1:8080")?
        .run()
        .await
}