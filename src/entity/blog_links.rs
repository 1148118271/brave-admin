use rbatis::crud::CRUD;
use rbatis::{DateTimeNative, Page, PageRequest};
use serde::Deserialize;
use serde::Serialize;
use crate::conf::mysql;


/// 友链
#[rbatis::crud_table]
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct BlogLinks {
    /// 主键
    pub id: Option<u64>,
    /// 链接名称
    pub link_name: Option<String>,
    /// 链接路径
    pub link_url: Option<String>,
    /// 是否启用
    pub flag: Option<String>,
    /// 创建时间
    pub create_time: Option<DateTimeNative>
}

impl BlogLinks {
    /// 获得分页友链列表
    pub async fn get_page_links_list(current_page: u64, page_size: u64) -> Option<Page<Self>> {
        let mysql = mysql::default().await;
        let wrapper = mysql.new_wrapper().eq("flag", "1").order_by(false, &["create_time"]);
        let pr = PageRequest::new(current_page, page_size);
        let result: rbatis::Result<Page<Self>> = mysql
            .fetch_page_by_wrapper(wrapper, &pr).await;
        match result {
            Ok(v) => Some(v),
            Err(e) => {
                log::error!("查询友链分页列表错误, 错误信息为: {}", e);
                None
            }
        }
    }
}