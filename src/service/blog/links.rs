/// 友链 service
/// 2022-04-19 21:59:09





use actix_web::HttpResponse;
use actix_web::web::Json;
use serde_json::Value;
use crate::entity::blog_links::BlogLinks;
use crate::{get_page, success, value};


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