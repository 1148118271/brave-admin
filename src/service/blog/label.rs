/// 博客标签 service
/// 2022-04-03 22:45:50






use actix_web::HttpResponse;
use actix_web::web::Json;
use rbatis::DateTimeNative;
use serde_json::Value;
use crate::entity::blog_label::BlogLabel;
use crate::{error, get_page, success, value};

/// 获得博客标签选择器列表
pub async fn get_label_select_list() -> HttpResponse {
    let vec = BlogLabel::get_label_list().await;
    let mut ss = vec![];
    if vec.is_empty() {
        return success!("成功!")
    }
    for bl in vec {
        ss.push(value! {"label" => bl.label_value, "value" => bl.label_key})
    }
    success!("成功!", ss)
}


/// 获得标签分页列表
pub async fn select_label_list(params: Json<Value>) -> HttpResponse {
    let mut result = Value::default();
    let (current, page_size) = get_page!(&params);
    let list = BlogLabel::get_page_label_list(current, page_size).await;
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
    for bl in page.records {
        let mut dov = value! {
            "id" => bl.id.unwrap_or(0),
            "label_key" => bl.label_key,
            "label_value" => bl.label_value,
        };
        if bl.create_time.is_some() {
            value! {dov; {"create_time" => bl.create_time.unwrap().to_string()}}
        }
        if bl.update_time.is_some() {
            value! {dov; {"update_time" => bl.update_time.unwrap().to_string()}}
        }
        vec.push(dov)
    }
    result["data"] = Value::Array(vec);
    success!("查询成功!", result)
}


/// 新增或修改标签信息
pub async fn add_or_update(mut b: BlogLabel) -> HttpResponse {
    b.update_time = Some(DateTimeNative::now());
    // 修改
    if b.id.is_some() {
        let result = BlogLabel::update(b).await;
        if let Err(e) = result {
            log::error!("修改标签信息异常, 异常信息为: {}", e);
            return error!("修改异常!");
        }
        return success!("修改数据成功!");
    }
    let key = BlogLabel::get_blog_label_by_key(&b.label_key).await;
    if key.is_some() {
        return error!("标识存在重复, 请修改!");
    }
    b.create_time = Some(DateTimeNative::now());
    let result = BlogLabel::save(b).await;
    match result {
        Ok(_) => success!("保存成功!"),
        Err(e) => {
            log::error!("保存标签信息异常, 异常信息为: {}", e);
            error!("保存失败!")
        }
    }
}


/// 删除标签
pub async fn delete_label(id: u64) -> HttpResponse {
    let result = BlogLabel::delete(id).await;
    match result {
        Ok(_) => success!("删除成功!"),
        Err(e) => {
            log::error!("删除标签信息异常, 异常信息为: {}", e);
            error!("保存失败!")
        }
    }
}