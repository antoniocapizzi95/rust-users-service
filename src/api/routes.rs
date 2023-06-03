use actix_web::{delete, get, post, put, web, HttpResponse, Responder};
use actix_web::web::Path;

use crate::models::user_model::{User};
use crate::repository::user_repository_mongo::UserRepository;
use crate::utils::generate_random_string::generate_random_string;

#[get("/users/{id}")]
pub async fn get_user(path: Path<String>, repository: web::Data<UserRepository>) -> impl Responder {
    let id = path.into_inner();
    if let Some(user) = repository.get_user(&id).await {
        HttpResponse::Ok().json(user)
    } else {
        HttpResponse::NotFound().finish()
    }
}

#[get("/users")]
pub async fn get_users(repository: web::Data<UserRepository>) -> HttpResponse {
    let users = repository.get_users().await;
    HttpResponse::Ok().json(users)
}

#[post("/users")]
pub async fn create_user(user: web::Json<User>, repository: web::Data<UserRepository>) -> HttpResponse {
    let mut new_user = user.into_inner();
    new_user.id = generate_random_string(20);
    let res = repository.create_user(new_user).await;
    HttpResponse::Created().json(res.unwrap())
}

#[put("/users/{id}")]
pub async fn update_user(path: Path<String>, user: web::Json<User>, repository: web::Data<UserRepository>) -> HttpResponse {
    let id = path.into_inner();
    if let Some(user) = repository.update_user(&id, user.into_inner()).await {
        HttpResponse::Ok().json(user)
    } else {
        HttpResponse::NotFound().finish()
    }
}

#[delete("/users/{id}")]
pub async fn delete_user(path: Path<String>, repository: web::Data<UserRepository>) -> HttpResponse {
    let id = path.into_inner();
    match repository.delete_user(&id).await {
        Ok(_) => HttpResponse::NoContent().finish(),
        Err(_) => HttpResponse::NotFound().finish(),
    }
}