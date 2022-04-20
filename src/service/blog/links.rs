/// 友链 service
/// 2022-04-19 21:59:09





use actix_web::HttpResponse;
use actix_web::web::Json;
use rbatis::DateTimeNative;
use serde_json::Value;
use crate::entity::blog_links::BlogLinks;
use crate::{error, get_page, success, value};


/// 友链分页查询
pub async fn get_page_links(params: Json<Value>) -> HttpResponse {
    let mut result = Value::default();
    let (current, page_size) = get_page!(&params);
    let list= BlogLinks::get_page_links_list(current, page_size).await;
    let mut page_obj = value! {
        "currentPage" => current,
        "pageSize" => page_size,
    };
    if list.is_none() {
        page_obj["total"] = Value::from(0);
        result["page"] = Value::from(page_obj);
        return success!("查询成功! ", result);
    }
    let page = list.unwrap();
    page_obj["total"] = Value::from(page.total);
    result["page"] = Value::from(page_obj);
    let mut vec = vec![];
    for v in page.records {
        let mut rv = value! {};
        if v.id.is_some() {
            value! {rv; {"id" => v.id.unwrap()}}
        }
        if v.link_name.is_some() {
            value! {rv; {"link_name" => v.link_name.unwrap()}}
        }
        if v.link_url.is_some() {
            value! {rv; {"link_url" => v.link_url.unwrap()}}
        }
        if v.flag.is_some() {
            value! {rv; {"flag" => v.flag.unwrap()}}
        }
        if v.create_time.is_some() {
            value! {rv; {"create_time" => v.create_time.unwrap().to_string()}}
        }
        vec.push(rv)
    }
    result["data"] = Value::Array(vec);
    success!("查询成功!", result)
}


/// 新增或者是修改
pub async fn add_or_update(mut v: BlogLinks) -> HttpResponse {
    // 修改
    if v.id.is_some() {
        return match BlogLinks::update(v).await {
            Ok(_) => success!("修改成功!"),
            Err(e) => {
                log::error!("修改友链信息异常, 异常信息为: {}", e);
                error!("修改异常")
            }
        }
    }
    // 新增
    v.create_time = Some(DateTimeNative::now());
    match BlogLinks::save(v).await {
        Ok(_) => success!("保存成功!"),
        Err(e) => {
            log::error!("保存友链信息异常, 异常信息为: {}", e);
            error!("保存失败!")
        }
    }
}


/// 删除友链
pub async fn delete_links(id: u64) -> HttpResponse {
    match BlogLinks::links_del(id).await {
        Ok(_) => success!("删除成功!"),
        Err(e) => {
            log::error!("删除友链信息异常, 异常信息为: {}", e);
            error!("保存失败!")
        }
    }
}