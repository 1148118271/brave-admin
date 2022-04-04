/// 博客基本信息 controller
/// 2022-04-03 22:40:01


use actix_web::post;
use actix_web::web::{Json, Form};
use serde_json::Value;
use crate::entity::blog_info::BlogInfo;
use crate::service::blog::info;
use crate::util::result::{ResultNoVal, ResultVal};


/// 获取博客基本信息列表
#[post("/blog/info")]
pub async fn get_blog_info(params: Json<Value>) -> Json<ResultVal<Value>> {
    info::blog_info(params).await
}

/// 博客信息新增
#[post("/blog/info/add")]
pub async fn add_blog_info(params: Json<BlogInfo>) -> Json<ResultNoVal>{
    info::blog_info_add(params).await
}
