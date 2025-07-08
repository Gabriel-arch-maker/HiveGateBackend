use actix_web::{post, web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};
use crate::routes::auth::signup;
use std::env;

use sea_orm::{ActiveModelTrait, Set, DbConn, DatabaseConnection};
use sea_orm::TryGetError::DbErr;
use crate::entities::user;
use uuid::Uuid;
use bcrypt::{hash, DEFAULT_COST};
#[derive(Serialize)]
struct SignupResponse {
    email: String,
    username: String,
    password: String,
}

#[derive(Deserialize)]
struct SignupData {
    email: String,
    username: String,
    password: String,
}

async fn create_user(db: &DatabaseConnection, signup_data: SignupData) ->  Result<SignupResponse, sea_orm::DbErr> {
    let new_user = user::ActiveModel{
        id: Set(Uuid::new_v4()),
        email: Set(signup_data.email.clone()),
        username: Set(signup_data.username.clone()),
        password: Set(hash(signup_data.password.clone(), DEFAULT_COST).unwrap()),
        ..Default::default()
    };
    let user = new_user.insert(db).await?;

   Ok(SignupResponse {
        email: signup_data.email.to_string(),
        username: signup_data.username.to_string(),
        password: signup_data.password.to_string(),
    })
}

#[post("/signup")]
async fn signup_handler(db: web::Data<DatabaseConnection>,data: web::Json<SignupData>) -> impl Responder {
    let user_data = data.into_inner();
    println!("got here");
    match create_user(db.get_ref(),user_data).await {
        Ok(user) => HttpResponse::Ok().json(user),
        Err(err) => HttpResponse::InternalServerError().body(format!("Error creating user: {:?}", err)),
    }
}

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(signup_handler);
}