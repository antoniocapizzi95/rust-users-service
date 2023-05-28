use actix_web::{delete, get, post, put, web, HttpResponse, Responder};
use actix_web::web::Path;
use serde::{Deserialize, Serialize};

use crate::models::{User};
use crate::repository::user_repository_mongo::UserRepository;

#[get("/users/{id}")]
pub async fn get_user(path: web::Path<String>, repository: web::Data<UserRepository>) -> impl Responder {
    let id = path.into_inner();
    if let Some(user) = repository.get_user(&id).await {
        HttpResponse::Ok().json(user)
    } else {
        HttpResponse::NotFound().finish()
    }
}

#[get("/users")]
pub async fn get_users() -> HttpResponse {
    let users = [
        User {
            id: 0.to_string(),
            name: String::from("Alice"), surname: String::from("Smith"),
            city: "New York".to_string(),
        },
        User {
            id: 1.to_string(),
            name: "Bob".to_string(), surname: "Red".to_string(),
            city: "Boston".to_string(),
        }
    ];
    HttpResponse::Ok().json(users)
}

#[post("/users")]
pub async fn create_user(user: web::Json<User>, repository: web::Data<UserRepository>) -> HttpResponse {
    let res = repository.create_user(user.into_inner()).await;
    HttpResponse::Created().json(res.unwrap())
}

#[put("/users/{id}")]
pub async fn update_user(path: Path<String>, user: web::Json<User>) -> HttpResponse {
    let id = path.into_inner();
    HttpResponse::Ok().json(User {
        id,
        name: user.name.clone(),
        surname: user.surname.clone(),
        city: user.city.clone(),
    })
}

#[delete("/users/{id}")]
pub async fn delete_user(path: Path<String>) -> HttpResponse {
    let id = path.into_inner();
    HttpResponse::NoContent().finish()
}