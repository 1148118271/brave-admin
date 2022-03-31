use rbatis::DateTimeNative;
use serde::Deserialize;
use serde::Serialize;

#[rbatis::crud_table]
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct BlogInfo {
    id: usize,
    title: String,
    label_key: Option<String>,
    is_publish: String,
    publish_time: Option<DateTimeNative>,
    read_count: usize,
    create_time: Option<DateTimeNative>,
    update_time: Option<DateTimeNative>
}

impl BlogInfo {
    
}