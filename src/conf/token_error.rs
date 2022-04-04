use actix_web::{error::ResponseError, HttpResponse};
use std::fmt::{Display, Formatter};
use actix_web::http::header;
use actix_web::http::header::HeaderValue;
use crate::util::result::ResultNoVal;

#[derive(Debug)]
pub struct TokenError {
    r: ResultNoVal
}

impl TokenError {
    pub fn new() -> TokenError {
        let val = ResultNoVal {
            code: 999,
            msg: "登录失效,请重新登录!".to_string()
        };
        Self::from(val)
    }
    fn from(r: ResultNoVal) -> TokenError {
        TokenError {
            r
        }
    }
}

impl Display for TokenError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.r)
    }
}

impl ResponseError for TokenError {
    fn error_response(&self) -> HttpResponse {
        let mut response = HttpResponse::Ok().json(&self.r);
        let h = response.headers_mut();
        h.append(header::ACCESS_CONTROL_ALLOW_ORIGIN, HeaderValue::from_static("*"));
        h.append(header::ACCESS_CONTROL_ALLOW_METHODS, HeaderValue::from_static("GET, OPTIONS, POST"));
        h.append(header::ACCESS_CONTROL_ALLOW_HEADERS, HeaderValue::from_static("*"));
        h.append(header::ACCESS_CONTROL_MAX_AGE, HeaderValue::from(3600));
        response
    }
}