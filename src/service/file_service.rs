/// 文件处理 service
/// 2022-04-05 16:19:10


use actix_web::web::Json;
use crate::success;
use crate::util::multipart_file::MultipartFile;
use crate::util::result::ResultNoVal;

pub async fn file_upload(file_path: String, file: MultipartFile) -> Json<ResultNoVal> {
    //println!("{:?}", file.file_name());
    success!("上传成功")
}