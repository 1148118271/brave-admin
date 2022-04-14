use std::future::{ready, Ready};
use actix_web::Error;
use actix_web::dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform};
use futures_util::future::LocalBoxFuture;
use crate::util::constant;

pub struct Auth;

impl<S, B> Transform<S, ServiceRequest> for Auth
    where
        S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
        S::Future: 'static,
        B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Transform = AuthMiddleware<S>;
    type InitError = ();
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(AuthMiddleware { service }))
    }
}

pub struct AuthMiddleware<S> {
    service: S,
}

impl<S, B> Service<ServiceRequest> for AuthMiddleware<S>
    where
        S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
        S::Future: 'static,
        B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    forward_ready!(service);

    fn call(&self, req: ServiceRequest) -> Self::Future {

        log::info!("[{}] => {}", req.method(), req.path());

        let mut flag = false;
        if let Ok(()) = path_validation(req.path()) {
            flag = true
        } else {
            let headers = req.headers();
            if let Some(v) = headers.get("token") {
                if let Ok(v) = v.to_str() {
                    if let Some(_) = super::session::get(v) {
                        flag = true
                    }
                }
            }
        }
        if !flag {
            log::error!("token匹配失败.");
            return Box::pin(async move {
                let e = super::token_error::TokenError::new();
                let error = Error::from(e);
                Err(error)
            })
        }
        let fut = self.service.call(req);

        log::info!("token匹配成功.");

        Box::pin(async move {
            let res = fut.await?;
            Ok(res)
        })
    }
}

/// 拦截器是否放过该请求
fn path_validation(path: &str) -> Result<(), ()> {
    if constant::RELEASE.contains(&path) {
        return Ok(())
    }
    for p in constant::RELEASE {
        let v: Vec<&str> = path.split("/").filter(|v| *v != "").collect();
        if v.is_empty() {
            return Err(())
        }
        let v = format!("{}/*", *&v[0]);
        if p.contains(&v) {
            return Ok(())
        }
    }
    return Err(())
}