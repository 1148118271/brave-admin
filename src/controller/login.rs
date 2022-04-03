//! 登录 controller
//! 2022-03-27 17:43:15
//! 高祥康

use actix_web::{HttpResponse, post};
use actix_web::web::Json;
use crate::entity::user::User;
use crate::util::result::ResultNoVal;
use crate::service::login_service;
use crate::conf::{config, session};


/// 登录
/// post
#[post("/login")]
pub async fn login(params: Json<User>) -> HttpResponse {
    login_service::login(params).await
}
