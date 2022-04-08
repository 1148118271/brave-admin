/// 博客基本信息 controller
/// 2022-04-03 22:40:01


use actix_web::{post, get, HttpResponse};
use actix_web::web::{Json, Path};
use serde_json::Value;
use crate::entity::blog_info::BlogInfo;
use crate::service::blog::info;

/// 获取博客基本信息列表
#[post("/blog/info")]
pub async fn get_blog_info(params: Json<Value>) -> HttpResponse {
    info::blog_info(params).await
}

/// 博客信息新增
#[post("/blog/info/add")]
pub async fn add_blog_info(params: Json<BlogInfo>) -> HttpResponse {
    info::blog_info_add(params).await
}

/// 删除博客信息
#[get("/blog/info/del/{id}")]
pub async fn del_blog_info(id: Path<u64>) -> HttpResponse {
    info::blog_info_del(*id).await
}