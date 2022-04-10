/// 帖子 controller
/// 2022-04-08 21:39:28

use actix_web::{HttpResponse, get, post};
use actix_web::web::{Json, Path};
use crate::entity::blog_post::BlogPost;
use crate::service;



/// 根据博客信息id查询帖子
#[get("/blog/post/get/{info_id}")]
pub async fn get_by_blog_info_id(info_id: Path<u64>) -> HttpResponse {
    service::blog::post::get_by_blog_info_id(info_id.into_inner()).await
}

/// 保存或者修改帖子
#[post("/blog/post/addOrUpdate")]
pub async fn add_or_update_post(v: Json<BlogPost>) -> HttpResponse {
    service::blog::post::add_or_update_post(v.into_inner()).await
}

/// 发布帖子
#[get("/blog/post/publish/{blog_info_id}")]
pub async fn publish(blog_info_id: Path<u64>) -> HttpResponse {
    service::blog::post::publish(blog_info_id.into_inner()).await
}

/// 删除帖子
#[get("/blog/post/del/{blog_info_id}")]
pub async fn delete_post(blog_info_id: Path<u64>) -> HttpResponse {
    service::blog::post::delete_post(blog_info_id.into_inner()).await
}