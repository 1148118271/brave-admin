use rbatis::rbatis::Rbatis;
use crate::conf::config;

pub static mut MYSQL: Option<Rbatis> = None;


pub fn default() -> &'static Rbatis {
    unsafe {
        if MYSQL.is_none() {
            actix_web::rt::spawn(async {
                let r = Rbatis::new();
                let conf = config::default();
                r.link(conf.database.url.as_str())
                    .await
                    .expect("数据库连接异常");
                MYSQL = Some(r)
            });
        }
        MYSQL.as_ref().unwrap()
    }
}
