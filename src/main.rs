mod controller;
mod entity;
mod util;
mod service;
mod conf;


use actix_cors::Cors;
use actix_web::{App, HttpServer};
use crate::conf::log::Log;
use crate::controller::{
    blog,
    index,
    login
};


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
            .service(index::index)
            .service(login::login)
            .service(blog::info::get_blog_info)
            .service(blog::info::add_blog_info)
            .service(blog::info::del_blog_info)
            .service(blog::label::get_label_select_list)
    }).bind(("0.0.0.0", 8000))
        .expect("项目启动失败!")
        .run()
        .await
        .expect("项目启动失败!")
}