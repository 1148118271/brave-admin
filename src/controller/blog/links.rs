/// 友链 controller
/// 2022-04-19 21:56:20




use actix_web::{HttpResponse, post};
use actix_web::web::Json;
use serde_json::Value;
use crate::service;


#[post("/blog/links/page")]
pub async fn get_page_links(params: Json<Value>) -> HttpResponse {
    service::blog::links::get_page_links(params).await
}