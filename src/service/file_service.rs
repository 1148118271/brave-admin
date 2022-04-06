/// 文件处理 service
/// 2022-04-05 16:19:10
///
///


use std::fs;
use std::io::ErrorKind::NotFound;
use std::io::Write;
use actix_web::web::Json;
use rbatis::DateTimeNative;
use crate::conf::config;
use crate::{error, os_path, success};
use crate::entity::files::Files;
use crate::util::multipart_file::MultipartFile;
use crate::util::result::ResultNoVal;

pub async fn file_upload(file_path: String, multipart_file: MultipartFile) -> Json<ResultNoVal> {
    let file_name = multipart_file.file_uuid_name();
    let result = fs::read_dir(&file_path);
    if let Err(ref e) = result {
        if e.kind() == NotFound {
            fs::create_dir_all(&file_path);
        } else {
            log::error!("读取文件夹异常, 文件夹地址为: {}, 异常信息为: {}", &file_path, e);
            return error!("获取文件夹异常!")
        }
    }
    let file_url = os_path!(&file_path, &file_name);
    let result = fs::OpenOptions::new()
        .create_new(true)
        .write(true)
        .open(&file_url);
    let mut file = match result {
        Ok(f) => f,
        Err(e) => {
            log::error!("创建文件异常, 文件路劲为: {}, 异常信息为: {}", &file_url, e);
            return error!("创建文件异常!")
        }
    };
    let result = file.write_all(multipart_file.bytes());
    if let Err(e) = result {
        log::error!("保存文件异常,异常信息为: {}", e);
        return error!("保存文件异常!")
    }

    Files {
        id: None,
        file_url: None,
        file_original_name: Some(multipart_file.file_original_name()),
        file_uuid_name: Some(file_name),
        file_size: Some(multipart_file.size() as u64),
        file_type: Some(multipart_file.file_type()),
        upload_time: Some(DateTimeNative::now())
    };


    success!("上传成功")
}

#[test] fn t() {
    // let result = read_dir("./upload");
    // if let Err(ref e) = result {
    //     if e.kind() == NotFound {
    //         println!("sss")
    //     }
    // }
    // println!("{:?}", result)
}