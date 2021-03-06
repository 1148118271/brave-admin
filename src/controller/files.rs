/// 统一文件上传下载删除
/// 2022-04-05 16:08:57




use actix_web::{HttpResponse, post};
use actix_multipart::Multipart;
use actix_web::web::{Json, Path};
use serde_json::Value;
use crate::{error, os_path};
use crate::conf::config;
use crate::service::files_service;
use crate::util::multipart_file::MultipartFile;


/// 单个文件上传
#[post("/file/upload/{path}")]
pub async fn file_upload(path: Path<String>, mut data: Multipart) -> HttpResponse {
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

    files_service::file_upload(file_path, r.unwrap()).await
}

/// 单个文件删除
#[post("/file/delete")]
pub async fn file_delete(v: Json<Value>) ->HttpResponse {
    let value = &v["file_path"];
    match value.as_str() {
        None => return error!("文件路径不能为空!"),
        Some(v) => {
            files_service::file_delete(v.to_string()).await
        }
    }

}