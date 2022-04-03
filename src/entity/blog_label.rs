use rbatis::crud::CRUD;
use rbatis::DateTimeNative;
use serde::Deserialize;
use serde::Serialize;
use crate::conf::mysql;


/// 标签信息
#[rbatis::crud_table]
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct BlogLabel {
    /// 主键
    pub id: u64,
    /// 标签key
    pub label_key: String,
    /// 标签value
    pub label_value: String,
    /// 创建时间
    pub create_time: Option<DateTimeNative>,
    /// 修改时间
    pub update_time: Option<DateTimeNative>,
}

impl BlogLabel {
    pub async fn get_blog_label_by_key(key: &str) -> Option<Self> {
        let mysql = mysql::default().await;
        let result: rbatis::Result<Option<Self>> = mysql.fetch_by_column("label_key", key).await;
        match result {
            Ok(v) => v,
            Err(e) => {
                log::error!("查询博客标签异常,异常信息: {}", e);
                None
            }
        }
    }
}