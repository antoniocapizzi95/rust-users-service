mod models;
mod repository;
mod utils;
mod api;

use actix_web::{App, HttpServer};
use actix_web::web::Data;
use crate::repository::user_repository_mongo::UserRepository;

use crate::api::routes::{create_user, delete_user, get_user, get_users, update_user};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let db = UserRepository::init().await;
    let db_data = Data::new(db);

    HttpServer::new(move || {
        App::new()
            .app_data(db_data.clone())
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