mod controller;
mod entity;
mod util;
mod service;
mod conf;


use actix_cors::Cors;
use actix_files::Files;
use actix_web::{App, HttpServer};
use crate::conf::config;
use crate::conf::log::Log;
use crate::controller::{blog, files, index, login};


#[actix_web::main]
async fn main() {
    // 初始化日志
    Log::init();
    log::info!("启动项目.");
    let conf = config::default();
    // 启动项目
    HttpServer::new(|| {
        App::new()
            .wrap(Cors::permissive())
            .wrap(conf::auth::Auth)
            .service(Files::new(util::constant::FILE_PATH,
                                &conf.file.file_storage_path))
            .service(index::index)
            .service(login::login)
            .service(files::file_upload)
            .service(files::file_delete)
            .service(blog::info::get_blog_info)
            .service(blog::info::get_blog_info_by_id)
            .service(blog::info::add_blog_info)
            .service(blog::info::edit_blog_info)
            .service(blog::info::del_blog_info)
            .service(blog::post::get_by_blog_info_id)
            .service(blog::post::add_or_update_post)
            .service(blog::post::publish)
            .service(blog::post::delete_post)
            .service(blog::label::get_label_select_list)
            .service(blog::label::select_label_list)
            .service(blog::label::add_or_update)
            .service(blog::label::delete_label)
            .service(blog::links::get_page_links)
            .service(blog::links::add_or_update)
            .service(blog::links::delete_links)
            .service(blog::comments::get_comments_by_blog_id)
    }).bind(("0.0.0.0", 8000))
        .expect("项目启动失败!")
        .run()
        .await
        .expect("项目启动失败!")
}