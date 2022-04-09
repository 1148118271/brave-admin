/// 帖子 service
/// 2022-04-09 12:32:15




use actix_web::HttpResponse;
use rbatis::DateTimeNative;
use crate::entity::blog_post::BlogPost;
use crate::{error, success};



/// 根据博客信息id查询帖子
pub async fn get_by_blog_info_id(info_id: u64) -> HttpResponse {
    let blog_post = BlogPost::query_by_blog_info_id(info_id).await;
    match blog_post {
        None => success!("查询成功!"),
        Some(v) => success!("查询成功!", v)
    }
}


/// 保存或者修改帖子
pub async fn add_or_update_post(mut v: BlogPost) -> HttpResponse {
    v.update_time = Some(DateTimeNative::now());
    if v.blog_info_id.is_none() {
        return error!("博客信息id不能为空!")
    }
    let blog_post = BlogPost::query_by_blog_info_id(v.blog_info_id.unwrap()).await;
    if let None = blog_post {
        v.create_time = Some(DateTimeNative::now());
        if let Err(e) = BlogPost::save(v).await {
            log::error!("保存帖子信息异常, 异常信息为:{}", e);
            return error!("保存帖子信息失败!")
        }
        return success!("保存帖子成功!")
    }
    if let Err(e) = BlogPost::update(v).await {
        log::error!("修改帖子信息异常, 异常信息为:{}", e);
        return error!("修改帖子信息失败!")
    }
    return success!("修改帖子成功!")
}