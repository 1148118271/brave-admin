use rbatis::rbatis::Rbatis;
use crate::conf::config;

static mut MYSQL: Option<Rbatis> = None;


pub async fn default() -> &'static Rbatis {
    unsafe {
        if MYSQL.is_none() {
            let r = Rbatis::new();
            let conf = config::default();
            r.link(conf.database.url.as_str()).await.expect("mysql连接异常");
            MYSQL = Some(r);
        }
        MYSQL.as_ref().unwrap()
    }
}
