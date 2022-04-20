/// 友链 controller
/// 2022-04-19 21:56:20




use actix_web::{HttpResponse, post, get};
use actix_web::web::{Json, Path};
use serde_json::Value;
use crate::entity::blog_links::BlogLinks;
use crate::service;


/// 友链分页查询
#[post("/blog/links/page")]
pub async fn get_page_links(params: Json<Value>) -> HttpResponse {
    service::blog::links::get_page_links(params).await
}


/// 新增或者修改友链信息
#[post("/blog/links/addOrUpdate")]
pub async fn add_or_update(v: Json<BlogLinks>) -> HttpResponse {
    service::blog::links::add_or_update(v.into_inner()).await
}


/// 删除友链
#[get("/blog/links/del/{id}")]
pub async fn delete_links(id: Path<u64>) -> HttpResponse {
    service::blog::links::delete_links(id.into_inner()).await
}