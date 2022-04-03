use rbatis::crud::CRUD;
use rbatis::{DateTimeNative, Page, PageRequest};
use serde::{Deserialize, Serialize};
use crate::conf::mysql;


/// 博客信息
#[rbatis::crud_table]
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct BlogInfo {
    /// 主键
    pub id: u64,
    /// 标题
    pub title: String,
    /// 标签
    pub label_key: Option<String>,
    /// 是否发布 是[`1`] 否[`0`]
    pub is_publish: String,
    /// 发表时间
    pub publish_time: Option<DateTimeNative>,
    /// 阅读次数
    pub read_count: u64,
    /// 创建时间
    pub create_time: Option<DateTimeNative>,
    /// 修改时间
    pub update_time: Option<DateTimeNative>
}

impl BlogInfo {
    /// 查询博客列表
    ///
    /// [`current_page`] 当前页数
    ///
    /// [`page_size`]  每页条数
    pub async fn get_blog_info(current_page: u64, page_size: u64) -> Option<Page<Self>> {
        let mysql = mysql::default().await;
        let wrapper = mysql.new_wrapper().order_by(false, &["create_time"]);
        let pr = PageRequest::new(current_page, page_size);
        let result: rbatis::Result<Page<Self>> = mysql
            .fetch_page_by_wrapper(wrapper, &pr).await;
        match result {
            Ok(v) => Some(v),
            Err(e) => {
                log::error!("查询博客信息错误, 错误信息为: {}", e);
                None
            }
        }
    }
}