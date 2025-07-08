use actix_web::{get, web, HttpResponse,Responder};
use serde::{ Deserialize, Serialize};
use crate::routes::users;

#[derive(Serialize)]
struct UserResponse {
    message: String,
    username: String,
}

#[get("/users/{username}")]
async fn get_all_users(path: web::Path<String>) -> impl Responder {
    let username = path.into_inner();
    let response = UserResponse {
        message: "Hello, world!".to_string(),
        username: "users".to_string(),
    };
    
    HttpResponse::Ok().json(response)
}