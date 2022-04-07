/// 拦截器放行的请求
pub const RELEASE: [&str; 3] = ["/", "/login", "/files/*"];

/// 静态文件统一路径
pub const FILE_PATH: &'static str = "files/";