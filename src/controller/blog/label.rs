/// 博客标签信息 controller
/// 2022-04-03 22:40:31


use actix_web::{HttpResponse, post, get};
use actix_web::web::{Json, Path};
use serde_json::Value;
use crate::entity::blog_label::BlogLabel;
use crate::service;

/// 获得博客标签选择器列表
#[post("/blog/label/select")]
pub async fn get_label_select_list() -> HttpResponse {
    service::blog::label::get_label_select_list().await
}


/// 获得标签分页列表
#[post("/blog/label/page/list")]
pub async fn select_label_list(params: Json<Value>) -> HttpResponse {
    service::blog::label::select_label_list(params).await
}


/// 新增或者修改标签信息
#[post("/blog/label/addOrUpdate")]
pub async fn add_or_update(v: Json<BlogLabel>) -> HttpResponse {
    service::blog::label::add_or_update(v.into_inner()).await
}


/// 删除标签
#[get("/blog/label/del/{id}")]
pub async fn delete_label(id: Path<u64>) -> HttpResponse {
    service::blog::label::delete_label(id.into_inner()).await
}