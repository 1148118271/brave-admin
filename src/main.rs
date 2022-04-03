mod controller;
mod entity;
mod util;
mod service;
mod conf;


use std::env;
use std::fs::{File, OpenOptions};
use actix_cors::Cors;
use actix_web::{App, http, HttpResponse, HttpServer};

use actix_web::web::Json;
use rbatis::log::LogPlugin;

use controller:: {
    index::index,
    login::login
};
use crate::conf::log::Log;
use crate::controller::blog::get_blog_info;


#[actix_web::main]
async fn main() {
    // 初始化日志
    Log::init();
    log::info!("启动项目.");
    // 启动项目
    HttpServer::new(|| {
        App::new()
            .wrap(Cors::permissive())
            .wrap(conf::auth::Auth)
            .service(index)
            .service(login)
            .service(get_blog_info)
    }).bind(("0.0.0.0", 8000))
        .expect("项目启动失败!")
        .run()
        .await
        .expect("项目启动失败!")
}