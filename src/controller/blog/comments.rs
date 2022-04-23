/// 博客评论 controller
/// 2022-04-23 14:06:25
use actix_web::{ HttpResponse, get };
use actix_web::web::Path;
use crate::service;

/// 获取博客的评论
#[get("/blog/comments/info/{id}")]
pub async fn get_comments_by_blog_id(id: Path<u64>) -> HttpResponse {
    service::blog::comments::get_comments_by_blog_id(id.into_inner()).await
}