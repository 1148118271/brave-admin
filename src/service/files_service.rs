/// 文件处理 service
/// 2022-04-05 16:19:10
///
///


use std::fs;
use std::io::ErrorKind::NotFound;
use std::io::Write;
use actix_web::HttpResponse;
use actix_web::web::Json;
use rbatis::DateTimeNative;
use crate::conf::config;
use crate::{error, os_path, success, util};
use crate::entity::files::Files;
use crate::util::multipart_file::MultipartFile;


/// 单个文件上传
pub async fn file_upload(file_path: String, multipart_file: MultipartFile) -> HttpResponse {
    let file_name = multipart_file.file_uuid_name();
    let result = fs::read_dir(&file_path);
    if let Err(ref e) = result {
        if e.kind() == NotFound {
            if let Err(e) = fs::create_dir_all(&file_path) {
                log::error!("创建文件夹异常, 异常信息为: {}", e);
                return error!("创建文件夹异常!")
            }
        } else {
            log::error!("读取文件夹异常, 文件夹地址为: {}, 异常信息为: {}", &file_path, e);
            return error!("获取文件夹异常!")
        }
    }
    let file_path = os_path!(&file_path, &file_name);
    let result = fs::OpenOptions::new()
        .create_new(true)
        .write(true)
        .open(&file_path);
    let mut file = match result {
        Ok(f) => f,
        Err(e) => {
            log::error!("创建文件异常, 文件路劲为: {}, 异常信息为: {}", &file_path, e);
            return error!("创建文件异常!")
        }
    };
    let result = file.write_all(multipart_file.bytes());
    if let Err(e) = result {
        log::error!("保存文件异常,异常信息为: {}", e);
        return error!("保存文件异常!")
    }

    let config = config::default();
    let file_storage_path = &config.file.file_storage_path;
    let file_url = file_path.clone().replace(file_storage_path, util::constant::FILE_PATH);

    let files = Files {
        id: None,
        file_url: Some(file_url.clone()),
        file_original_name: Some(multipart_file.file_original_name()),
        file_uuid_name: Some(file_name),
        file_size: Some(multipart_file.size() as u64),
        file_type: Some(multipart_file.file_type()),
        upload_time: Some(DateTimeNative::now())
    };
    if let Err(e) = Files::save(files).await {
        log::error!("文件信息保存数据库异常,异常信息为: {}", e);
        match fs::remove_file(file_path) {
           _ => {}
        }
        return error!("文件信息保存数据库")
    }
    success!("上传成功", file_url)
}



/// 单个文件删除
pub async fn file_delete(file_path: String) -> HttpResponse {
    let vec = file_path.split("/").collect::<Vec<&str>>();
    let file_uuid_name = vec[vec.len() - 1];
    if let Err(e) = db_file_delete(file_uuid_name).await {
        log::error!("删除数据库文件信息异常, 异常信息为: {}", e);
        return error!("删除数据库文件信息异常")
    }
    let config = config::default();
    let file_storage_path = &config.file.file_storage_path;
    let file_path = file_path.replace(util::constant::FILE_PATH, file_storage_path);
    if let Err(e) = disk_file_delete(file_path).await {
        log::error!("删除磁盘文件信息异常, 异常信息为: {}", e);
        return error!("删除磁盘文件信息异常")
    }
    return success!("删除文件成功")
}

/// 磁盘文件删除
async fn disk_file_delete(file_path: String) -> std::io::Result<()> {
    fs::remove_file(file_path)
}

/// 数据库文件删除
async fn db_file_delete(file_uuid_name: &str) -> rbatis::Result<u64> {
    Files::delete_by_uuid_name(file_uuid_name).await
}