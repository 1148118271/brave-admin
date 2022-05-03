use actix_web::HttpResponse;
use rbatis::DateTimeNative;
use crate::entity::blog_post::BlogPost;
use crate::{error, success};
use crate::conf::cached;
use crate::entity::blog_info::BlogInfo;



/// 帖子 service
/// 2022-04-09 12:32:15

/// 发布帖子
pub async fn publish(blog_info_id: u64) -> HttpResponse {
    if let None = BlogPost::query_by_blog_info_id(blog_info_id).await {
       return error!("编写帖子后才能发布!");
    }
    let info = BlogInfo {
        id: Some(blog_info_id),
        title: None,
        label_key: None,
        is_publish: Some(String::from("1")),
        publish_time: Some(DateTimeNative::now()),
        read_count: None,
        create_time: None,
        update_time: None
    };
    match BlogInfo::blog_info_edit(info).await {
        Ok(_) => success!("发布成功!"),
        Err(e) => {
            log::error!("博客信息修改失败, 异常信息为: {}", e);
            error!("发布失败!")
        }
    }
}


/// 根据博客信息id删除帖子
pub async fn delete_post(blog_info_id: u64) -> HttpResponse {
    match BlogPost::delete_by_blog_info_id(blog_info_id).await {
        Ok(_) => success!("帖子删除成功!"),
        Err(e) => {
            log::error!("删除帖子异常, 异常信息为: {}", e);
            error!("帖子删除失败!")
        }
    }
}


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
    let ph = match &v.post_html {
        None => String::new(),
        Some(s) => s.clone()
    };
    v.update_time = Some(DateTimeNative::now());
    if v.blog_info_id.is_none() {
        return error!("博客信息id不能为空!")
    }
    let blog_post = BlogPost::query_by_blog_info_id(v.blog_info_id.unwrap()).await;
    let phk = format!("{}_{}", cached::POST_HTML_KEY, v.blog_info_id.unwrap());
    if let None = blog_post {
        v.create_time = Some(DateTimeNative::now());
        if let Err(e) = BlogPost::save(v).await {
            log::error!("保存帖子信息异常, 异常信息为:{}", e);
            return error!("保存帖子信息失败!")
        }
    } else {
        if let Err(e) = BlogPost::update(v).await {
            log::error!("修改帖子信息异常, 异常信息为:{}", e);
            return error!("修改帖子信息失败!")
        }
    }
    let cached = cached::default();
    cached.set(&phk, &ph).expect("添加缓存信息失败");
    return success!("操作成功!")
}