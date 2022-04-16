//! 用户登录
//! 2022-03-27 17:39:26
//! 高祥康




use actix_web::HttpResponse;
use actix_web::web::Json;
use crate::conf::{config, session};
use crate::entity::user::User;
use crate::util::result::ResultNoVal;


/// 登录
pub async fn login(params: Json<User>) -> HttpResponse {
    let (flag, msg) = validation(&*params);
    let mut builder = HttpResponse::Ok();
    if !flag {
        return builder.json(ResultNoVal::error(msg))
    }
    let digest = md5::compute(params.password.as_ref().unwrap().as_str());
    let md5_str = format!("{:x}", digest);
    session::set(md5_str.clone(), "admin".to_string());
    builder.append_header(("token", md5_str));
    builder.json(ResultNoVal::success("登录成功"))
}

/// 验证账号密码
fn validation<'a>(user: &User) -> (bool, &'a str) {
    let username = &user.username;
    let password = &user.password;
    if username.is_none() {
        return (false, "用户名不能为空!")
    }
    if password.is_none() {
        return (false, "密码不能为空!")
    }
    let conf = config::default();
    let l_username = &conf.user.username;
    let l_password = &conf.user.password;
    if l_username.as_ref().unwrap() != username.as_ref().unwrap() {
        return (false, "用户名或密码不匹配!")
    }
    if l_password.as_ref().unwrap() != password.as_ref().unwrap() {
        return (false, "用户名或密码不匹配!")
    }
    (true, "")
}