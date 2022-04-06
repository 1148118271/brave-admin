use rbatis::crud::CRUD;
use rbatis::DateTimeNative;
use rbatis::db::DBExecResult;
use serde::{Deserialize, Serialize};
use crate::conf::mysql;

/// 文件信息
#[rbatis::crud_table]
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Files {
    /// 主键
    pub id: Option<u64>,
    /// 文件路径
    pub file_url: Option<String>,
    /// 文件原始名称
    pub file_original_name: Option<String>,
    /// uuid后的名称
    pub file_uuid_name: Option<String>,
    /// 文件大小
    pub file_size: Option<u64>,
    /// 文件类型
    pub file_type: Option<String>,
    /// 上传时间
    pub upload_time: Option<DateTimeNative>
}

impl Files {
    pub async fn save(file: Files) -> rbatis::Result<DBExecResult> {
        let mysql = mysql::default().await;
        mysql.save(&file, &[]).await
    }
}
