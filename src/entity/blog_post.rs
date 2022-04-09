use rbatis::crud::CRUD;
use rbatis::DateTimeNative;
use rbatis::db::DBExecResult;
use serde::Deserialize;
use serde::Serialize;
use crate::conf::mysql;

/// 博客帖子详情
#[rbatis::crud_table]
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct BlogPost {
    /// 主键
    pub id: Option<u64>,
    /// 博客信息id
    pub blog_info_id: Option<u64>,
    /// 文本
    pub post_text: Option<String>,
    /// html
    pub post_html: Option<String>,
    /// 创建时间
    pub create_time: Option<DateTimeNative>,
    /// 修改时间
    pub update_time: Option<DateTimeNative>,
}

impl BlogPost {

    /// 根据博客信息关联id查询帖子
    pub async fn query_by_blog_info_id(blog_info_id: u64) -> Option<Self> {
        let mysql = mysql::default().await;
        let result: rbatis::Result<Self> = mysql.fetch_by_column("blog_info_id", blog_info_id).await;
        match result {
            Ok(v) => Some(v),
            Err(e) => {
                log::error!("查询帖子详情异常,异常信息: {}", e);
                None
            }
        }
    }

    /// 新增帖子
    pub async fn save(bp: BlogPost) -> rbatis::Result<DBExecResult> {
        let mysql = mysql::default().await;
        mysql.save(&bp, &[]).await
    }

    /// 修改帖子
    pub async fn update(bp: BlogPost) -> rbatis::Result<u64> {
        let mysql = mysql::default().await;
        mysql.update_by_column("id", &bp).await
    }
}