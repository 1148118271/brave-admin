use std::collections::HashMap;
use actix_web::post;
use actix_web::web::Json;
use serde_json::Value;
use crate::service::blog_service;
use crate::success;
use crate::util::result::{ResultNoVal, ResultVal};


#[post("/blog/info")]
pub async fn get_blog_info(params: Json<Value>) -> Json<ResultVal<Value>> {
    blog_service::blog_info(params).await
}