mod routes;
mod models;

use actix_web::{web, App, HttpResponse, HttpServer, Responder};

use crate::routes::{create_user, delete_user, get_user, get_users, update_user};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
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