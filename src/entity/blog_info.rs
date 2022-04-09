use rbatis::crud::{CRUD, Skip};
use rbatis::{DateTimeNative, Page, PageRequest};
use rbatis::db::DBExecResult;
use serde::{Deserialize, Serialize};
use crate::conf::mysql;


/// 博客信息
#[rbatis::crud_table]
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct BlogInfo {
    /// 主键
    pub id: Option<u64>,
    /// 标题
    pub title: Option<String>,
    /// 标签
    pub label_key: Option<String>,
    /// 是否发布 是[`1`] 否[`0`]
    pub is_publish: Option<String>,
    /// 发表时间
    pub publish_time: Option<DateTimeNative>,
    /// 阅读次数
    pub read_count: Option<u64>,
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


    /// 根据id查询
    pub async fn get_blog_info_by_id(id: u64) -> Option<Self> {
        let mysql = mysql::default().await;
        let result: rbatis::Result<Option<Self>> = mysql.fetch_by_column("id", id).await;
        match result {
            Ok(v) => v,
            Err(e) => {
                log::error!("根据id查询博客信息错误, 错误信息为: {}", e);
                None
            }
        }
    }

    /// 新增博客信息
    pub async fn blog_info_add(b :BlogInfo) -> rbatis::Result<DBExecResult> {
        let mysql = mysql::default().await;
        mysql.save(&b, &[Skip::Column("id")]).await
    }


    /// 修改博客信息
    pub async fn blog_info_edit(b :BlogInfo) -> rbatis::Result<u64> {
        let mysql = mysql::default().await;
        mysql.update_by_column("id", &b).await
    }


    /// 根据id删除
    pub async fn blog_info_del(id: u64) -> rbatis::Result<u64> {
        let mysql = mysql::default().await;
        mysql.remove_by_column::<Self, u64>("id", id).await
    }
}