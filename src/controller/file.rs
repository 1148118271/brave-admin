use std::ffi::{OsStr, OsString};
use std::path;
/// 统一文件上传下载删除
/// 2022-04-05 16:08:57


use actix_web::{FromRequest, HttpRequest, post, web};
use actix_multipart::Multipart;
use actix_web::web::{Json, Payload, Path};
use serde_json::Value;
use crate::{error, os_path, success};
use crate::conf::config;
use crate::service::file_service;
use crate::util::multipart_file::MultipartFile;
use crate::util::result::ResultNoVal;


/// 单个文件上传
#[post("/file/upload/{path}")]
pub async fn file_upload(path: Path<String>, mut data: Multipart) -> Json<ResultNoVal> {
    let r = MultipartFile::init(&mut data).await;
    if r.is_none() {
        return error!("文件上传失败!")
    }

    let path = path.into_inner();
    let path = if cfg!(windows) {
        path.replace(".", "\\")
    } else {
        path.replace(".", "/")
    };

    let config = config::default();
    let file_storage_path = &config.file.file_storage_path;
    let file_path = os_path!(file_storage_path, &path);

    file_service::file_upload(file_path, r.unwrap()).await
}