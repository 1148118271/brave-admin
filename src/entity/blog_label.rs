use rbatis::crud::CRUD;
use rbatis::{DateTimeNative, Page, PageRequest};
use rbatis::db::DBExecResult;
use serde::Deserialize;
use serde::Serialize;
use crate::conf::mysql;


/// 标签信息
#[rbatis::crud_table]
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct BlogLabel {
    /// 主键
    pub id: Option<u64>,
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

    /// 根据标签标识获取标签详情
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

    /// 获取标签列表
    pub async fn get_label_list() -> Vec<Self> {
        let mysql = mysql::default().await;
        let result: rbatis::Result<Vec<Self>> = mysql.fetch_list().await;
        match result {
            Ok(v) => v,
            Err(e) => {
                log::error!("查询博客标签列表异常,异常信息: {}", e);
                vec![]
            }
        }
    }

    /// 获得分页标签列表
    pub async fn get_page_label_list(current_page: u64, page_size: u64) -> Option<Page<Self>> {
        let mysql = mysql::default().await;
        let wrapper = mysql.new_wrapper().order_by(false, &["create_time"]);
        let pr = PageRequest::new(current_page, page_size);
        let result: rbatis::Result<Page<Self>> = mysql
            .fetch_page_by_wrapper(wrapper, &pr).await;
        match result {
            Ok(v) => Some(v),
            Err(e) => {
                log::error!("查询标签列表错误, 错误信息为: {}", e);
                None
            }
        }
    }

    /// 根据id修改标签信息
    pub async fn update(v: BlogLabel) -> rbatis::Result<u64> {
        let mysql = mysql::default().await;
        mysql.update_by_column("id", &v).await
    }

    /// 新增标签信息
    pub async fn save(v: BlogLabel) -> rbatis::Result<DBExecResult> {
        let mysql = mysql::default().await;
        mysql.save(&v, &[]).await
    }

    /// 删除
    pub async fn delete(id: u64) -> rbatis::Result<u64> {
        let mysql = mysql::default().await;
        mysql.remove_by_column::<Self, u64>("id", id).await
    }
}