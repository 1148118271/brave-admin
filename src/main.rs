mod controller;
mod entity;
mod config;
mod auth;
mod session;
mod util;
mod service;

use actix_cors::Cors;
use actix_web::{App, http, HttpResponse, HttpServer};

use actix_web::web::Json;

use controller:: {
    index::index,
    login::login
};


#[actix_web::main]
async fn main() {
    HttpServer::new(|| {
        App::new()
            .wrap(Cors::permissive())
            .wrap(auth::Auth)
            .service(index)
            .service(login)
    }).bind(("0.0.0.0", 8000))
        .expect("项目启动失败!")
        .run()
        .await
        .expect("项目启动失败!")
}