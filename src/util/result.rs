use actix_web::web::Json;
use serde::Deserialize;
use serde::Serialize;

#[derive(Deserialize, Serialize, Debug)]
pub struct ResultNoVal {
    pub code: u16,
    pub msg: String,
}



impl ResultNoVal  {
    pub fn success(msg: &str) -> Self {
        ResultNoVal {
            code: 200,
            msg: msg.to_string(),
        }
    }

    pub fn error(msg: &str) -> Self {
        ResultNoVal {
            code: 500,
            msg: msg.to_string(),
        }
    }

}

#[derive(Deserialize, Serialize, Debug)]
pub struct ResultVal<T> {
    pub code: u16,
    pub msg: String,
    pub data: T
}


impl<T> ResultVal<T> {
    pub fn success(msg: &str, data: T) -> Self {
        let val = ResultNoVal::success(msg);
        ResultVal {
            code: val.code,
            msg: val.msg,
            data
        }
    }
}

#[macro_export]
macro_rules! success {
    ($msg:expr,$data:expr) => {
        return actix_web::web::Json(crate::util::result::ResultVal::success($msg, $data))
    };
    ($msg:expr) => {
        return actix_web::web::Json(crate::util::result::ResultNoVal::success($msg))
    };
    () => {
        return actix_web::web::Json(crate::util::result::ResultNoVal::success("成功!"))
    }
}

#[macro_export]
macro_rules! error {
    () => {
        return actix_web::web::Json(crate::util::result::ResultNoVal::error("失败!"))
    };
    ($msg:expr) => {
        return actix_web::web::Json(crate::util::result::ResultNoVal::error($msg))
    };
}
