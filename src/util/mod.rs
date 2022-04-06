pub mod result;
pub mod multipart_file;
pub mod constant;


/// 根据 [`Json(Value)`] 获取当前页数和每页条数
///
/// params [`Json(Value)`]
///
/// return ([`current_page`], [`page_size`])
///
/// current_page = 当前页数
/// page_size = 每页条数
#[macro_export]
macro_rules! get_page {
    ($v:expr) => {
       {
            let current_page = &$v["current_page"].as_u64().unwrap_or(1);
            let page_size = &$v["page_size"].as_u64().unwrap_or(10);
            (*current_page, *page_size)
        }
    }
}

/// 更方便的方式生成Value
/// 第一个匹配项
/// ```
/// value!{"k1" => 1, "k2" => 2}
/// ```
/// 等同于
/// ```
/// let mut value = serde_json::Value::default();
/// value["k1"] = serde_json::Value::from(1);
/// value["k2"] = serde_json::Value::from(2);
/// ```
///
/// 第二个匹配项
/// ```
/// let v = value!{"k1" => 1, "k2" => 2}
/// value! (v; {"k3" => 3, "k4" => 4});
/// ```
/// 等同于
/// ```
/// let mut value = serde_json::Value::default();
/// value["k1"] = serde_json::Value::from(1);
/// value["k2"] = serde_json::Value::from(2);
/// value["k3"] = serde_json::Value::from(3);
/// value["k4"] = serde_json::Value::from(4);
/// ```
#[macro_export]
macro_rules! value {
    () => {
        serde_json::Value::default()
    };
    ($($k:expr=>$v:expr),*$(,)?) => {
        {
            let mut value = serde_json::Value::default();
            $(
                value[$k] = serde_json::Value::from($v);
            )*
            value
        }
    };
    ($val:expr;{$($k:expr=>$v:expr),*$(,)?}) => {
         $(
            $val[$k] = serde_json::Value::from($v);
         )*
    }
}


#[macro_export]
macro_rules! os_path {
    ($($x:expr),*) => {{
        let mut buf = std::path::PathBuf::new();
        $(
            buf = buf.join(std::path::Path::new($x));
        )*
        buf.into_os_string().into_string().unwrap()
    }};
}