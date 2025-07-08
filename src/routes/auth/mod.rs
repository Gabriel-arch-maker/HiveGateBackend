mod login;
mod signup;

use actix_web::web;

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("/auth").configure(login::init_routes).configure(signup::init_routes));
}