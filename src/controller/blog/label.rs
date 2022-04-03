/// 博客标签信息 controller
/// 2022-04-03 22:40:31


use actix_web::web::Json;
use actix_web::post;
use serde_json::Value;
use crate::service;
use crate::util::result::ResultVal;

/// 获得博客标签选择器列表
#[post("/blog/label/select")]
pub async fn get_label_select_list() -> Json<ResultVal<Vec<Value>>> {
    service::blog::label::get_label_select_list().await
}