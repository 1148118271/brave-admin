use rbatis::crud::CRUD;
use rbatis::DateTimeNative;
use rbson::Bson;
use serde::{Deserialize, Serialize};
use crate::conf::mysql;

/// 评论信息
#[rbatis::crud_table]
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct BlogComments {
    /// 主键
    id: u64,
    /// 博客关联id
    blog_id: u64,
    /// 名字
    name: String,
    /// 邮箱
    email: Option<String>,
    /// 网址
    url: Option<String>,
    /// 评论
    comment: String,
    /// 发表时间
    create_time: Option<DateTimeNative>
}

impl BlogComments {
    /// 根据博客id查询评论数
    pub async fn get_blog_comments_by_blog_id(blog_id: u64) -> u64 {
        let r = mysql::default().await;
        let sql = r#"select count(1) from blog_comments where blog_id = ?"#;
        let result: rbatis::Result<u64> = r.fetch(sql, vec![Bson::UInt64(blog_id)]).await;
        match result {
            Ok(v) => v,
            Err(e) => {
                log::error!("查询评论列表异常,异常信息为:{}", e);
                0
            }
        }
    }
}