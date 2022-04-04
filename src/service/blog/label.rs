use actix_web::web::Json;
use crate::entity::blog_label::BlogLabel;
use crate::{success, value};
use crate::util::result::ResultVal;

/// 博客标签 service
/// 2022-04-03 22:45:50


/// 获得博客标签选择器列表
pub async fn get_label_select_list() -> Json<ResultVal<Vec<serde_json::Value>>> {
    let vec = BlogLabel::get_label_list().await;
    let mut ss = vec![];
    if vec.is_empty() {
        return success!("成功!", ss)
    }
    for bl in vec {
        ss.push(value! {"label" => bl.label_value, "value" => bl.label_key})
    }
    success!("成功!", ss)
}