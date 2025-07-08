use actix_web::{post, web, HttpResponse, Responder,};
use serde::{Serialize, Deserialize};
use crate::entities::user as User;
use sea_orm::prelude::*;
use bcrypt::verify;
#[derive(Serialize)]
struct LoginResponse {
    success: bool,
    message: String,
}

#[derive(Deserialize)]
pub struct LoginData {
    password: String,
    email: String,
}

#[post("/login")]
async fn login_handler(db: web::Data<DatabaseConnection>, data: web::Json<LoginData>) -> impl Responder {
    let login_data = data.into_inner();
    let user = User::Entity::find().filter(User::Column::Email.eq(login_data.email.clone())).one(db.get_ref()).await;
    
    match user {
        Ok(Some(user)) => {
            let is_password_valid = verify(&login_data.password, &user.password);
            match is_password_valid {
                Ok(true) => {
                    let response = LoginResponse {
                        success: true,
                        message: "Successfully Logged In".to_string()
                    };
                    HttpResponse::Ok().json(response)
                }
                Ok(false) => {
                    let response = LoginResponse {
                        success: false,
                        message: "Password Verification Failed".to_string(),
                    };
                    HttpResponse::Ok().json(response)
                }
                Err(err) => {
                    let response = LoginResponse {
                        success: false,
                        message: format!("Password check failed: {}", err),
                    };
                    HttpResponse::Ok().json(response)
                }
            }
            
        }
        Ok(None) => {
            let response = LoginResponse {
                success: false,
                message: "User not found".to_string(),
            };
            HttpResponse::Ok().json(response)
        }
        Err(err) => {
            let response = LoginResponse {
                success: false,
                message: format!("Database error: {}", err.to_string())
            };
            HttpResponse::InternalServerError().json(response)
        }
    }
}

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(login_handler);
}