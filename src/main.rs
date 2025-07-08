mod routes;
mod entities;
use sea_orm::Database;
use std::fs;
use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder, Scope, Result};
use actix_web_actors::ws;
use actix::{Actor,StreamHandler};
use dotenvy::dotenv;
use std::env;


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let db = Database::connect(&database_url).await.expect("Failed to connect to database");
    let db_data = web::Data::new(db);
    HttpServer::new(move || {App::new().app_data(db_data.clone()).configure(routes::auth::init_routes).configure(routes::ws::init_ws_routes)}).bind(("127.0.0.1", 8000))?.run().await    
}