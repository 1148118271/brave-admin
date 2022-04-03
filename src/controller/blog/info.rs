/// 博客基本信息 controller
/// 2022-04-03 22:40:01


use std::collections::HashMap;
use actix_web::post;
use actix_web::web::Json;
use serde_json::Value;
use crate::service::blog::info;
use crate::success;
use crate::util::result::{ResultNoVal, ResultVal};


/// 获取博客基本信息列表
#[post("/blog/info")]
pub async fn get_blog_info(params: Json<Value>) -> Json<ResultVal<Value>> {
    info::blog_info(params).await
}


