/// 博客基本信息 service
/// 2022-04-03 22:45:34







use actix_web::HttpResponse;
use actix_web::web::Json;
use rbatis::DateTimeNative;
use serde_json::Value;
use crate::entity::blog_info::BlogInfo;
use crate::{error, get_page, success, value};
use crate::entity::blog_comments::BlogComments;
use crate::entity::blog_label::BlogLabel;


/// 删除博客信息
pub async fn blog_info_del(id: u64) -> HttpResponse {
    if let Err(e) = BlogInfo::blog_info_del(id).await {
        log::error!("删除博客信息异常,异常信息为:{}", e);
        return error!("删除异常")
    }
    success!("删除成功")
}

/// 新增博客信息
pub async fn blog_info_add(mut params: Json<BlogInfo>) -> HttpResponse {
    // 是否发布 默认为否
    params.is_publish = Some("0".to_string());
    // 阅读次数
    params.read_count = Some(0);
    // 创建时间
    params.create_time = Some(DateTimeNative::now());
    // 修改时间
    params.update_time = Some(DateTimeNative::now());

    if let Err(e) = BlogInfo::blog_info_add(params.into_inner()).await {
        log::error!("博客信息新增异常, 异常信息为: {}", e);
        return error!("保存异常")
    }
    success!("保存成功")
}

/// 修改博客信息
pub async fn blog_info_edit(mut params: Json<BlogInfo>) -> HttpResponse {
    if params.id.is_none() {
        return error!("id 不能为空")
    }
    // 修改时间
    params.update_time = Some(DateTimeNative::now());
    if let Err(e) = BlogInfo::blog_info_edit(params.into_inner()).await {
        log::error!("博客信息修改异常, 异常信息为: {}", e);
        return error!("修改异常")
    }
    success!("修改成功")
}

/// 根据id获取博客信息
pub async fn blog_info_by_id(id: u64) -> HttpResponse {
    let v = match BlogInfo::get_blog_info_by_id(id).await {
        None => return error!("博客信息id有误,未查到数据"),
        Some(v) => v
    };
    if v.label_key.is_none() {
        return success!("成功", v);
    }
    let label_key_arr = v.label_key.as_ref().unwrap().split(",")
        .map(|k| Value::String(k.to_string())).collect();
    let mut result = match serde_json::to_value(v) {
        Ok(v) => v,
        Err(e) => {
            log::error!("转换json异常,异常信息为: {}", e);
            return error!("博客信息数据获取异常")
        }
    };
    result["label_key"] = Value::Array(label_key_arr);
    success!("成功", result)
}

/// 获取博客列表
pub async fn blog_info(params: Json<Value>) -> HttpResponse {
    let mut result = Value::default();
    let (current, page_size) = get_page!(&params);
    let mut rpage = value! {};
    // 当前页数
    rpage["currentPage"] = Value::from(current);
    // 每页条数
    rpage["pageSize"] = Value::from(page_size);

    let blog_info = BlogInfo::get_blog_info(current, page_size).await;
    if blog_info.is_none() {
        // 查询结果为空的话总条数为0
        rpage["total"] = Value::from(0);
        result["page"] = Value::from(rpage);
        return success!("查询成功", result)
    }

    let page = blog_info.unwrap();
    // 总条数
    rpage["total"] = Value::from(page.total);
    result["page"] = Value::from(rpage);
    assembly_blog_info(&mut result, page.records).await;
    return success!("查询成功", result)
}

/// 组装博客列表数据
async fn assembly_blog_info(result: &mut Value, blog_infos: Vec<BlogInfo>) {
    let mut vs = vec![];
    for b in blog_infos {
        let mut v = value! {
            "id" => b.id.unwrap_or(0),
            "title" => b.title.unwrap_or(String::new()),
            "is_publish" => b.is_publish.unwrap_or("0".to_string()),
            "read_count" => b.read_count.unwrap_or(0),
            "comments_count" => get_blog_comments(b.id.unwrap_or(0)).await,
        };

        if let Some(pt) = b.publish_time {
            value! {v; {"publish_time" => pt.to_string()}}
        }
        if let Some(ct) = b.create_time {
            value! {v; {"create_time" => ct.to_string()}}
        }
        if let Some(ut) = b.update_time {
            value! {v; {"update_time" => ut.to_string()}}
        }

        if let Some(k) = &b.label_key {
            let vec = get_label_value(k).await;
            value! {v; {"label_key" => vec}}
        }
        vs.push(v);
    }

    value! {result; {"data" => vs}}
}

/// 获得评论次数
async fn get_blog_comments(id: u64) -> u64 {
    BlogComments::get_blog_comments_by_blog_id(id).await
}

/// 获得标签名称
async fn get_label_value(label: &String) -> Vec<String> {
    let mut v = vec![];
    if label.is_empty() {
        return v
    }
    let ks = label.split(",").collect::<Vec<&str>>();
    for k in ks {
        if let Some(bl) = BlogLabel::get_blog_label_by_key(k).await {
            v.push(bl.label_value);
        }
    }
    v
}
