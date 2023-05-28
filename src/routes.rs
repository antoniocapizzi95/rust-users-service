use actix_web::{delete, get, post, put, web, HttpResponse, Responder};
use actix_web::web::Path;
use serde::{Deserialize, Serialize};

use crate::models::{User};

#[get("/users/{id}")]
pub async fn get_user(path: Path<String>) -> HttpResponse {
    let id = path.into_inner();
    HttpResponse::Ok().json(User {
        id,
        name: String::from("Bob"),
        surname: String::from("Ross"),
        city: String::from("Florida"),
    })
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
pub async fn create_user(user: web::Json<User>) -> HttpResponse {
    HttpResponse::Created().json(user.0)
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