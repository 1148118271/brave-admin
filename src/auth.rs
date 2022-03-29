use std::future::{ready, Ready};
use actix_cors::Cors;
use actix_web::{Error, HttpResponse, ResponseError};
use actix_web::dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform};
use actix_web::cookie::Expiration::Session;
use actix_web::http::StatusCode;
use futures_util::future::LocalBoxFuture;
use crate::{error, session, token_error};
use crate::util::result::ResultNoVal;

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
        let mut flag = false;
        println!("path => {}", req.path());
        if req.path() == "/login" || req.path() == "/" {
            flag = true
        } else {
            let headers = req.headers();
            if let Some(v) = headers.get("token") {
                if let Ok(v) = v.to_str() {
                    if let Some(_) = session::get(v) {
                        flag = true
                    }
                }
            }
        }

        let fut = self.service.call(req);
        Box::pin(async move {
            let res = fut.await?;
            if flag {
                return Ok(res)
            }
            let e = token_error::TokenError::new();
            let error = Error::from(e);
            Err(error)
        })
    }
}
