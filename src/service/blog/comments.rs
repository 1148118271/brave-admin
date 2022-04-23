/// 博客评论 service
/// 2022-04-23 14:05:57
use actix_web::HttpResponse;
use crate::entity::blog_comments::BlogComments;
use crate::success;

/// 获取博客评论
pub async fn get_comments_by_blog_id(id: u64) -> HttpResponse {
    let vec = BlogComments::get_comments_by_blog_id(id).await;
    success!("查询成功!", vec)
}