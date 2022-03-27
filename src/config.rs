use std::fs;
use serde::Deserialize;
use crate::entity::user::User;


pub static mut CONFIG: Option<Config> = None;


pub fn default() -> &'static Config {
    unsafe {
        match &CONFIG {
            None => {
                CONFIG = Some(Config::default());
                CONFIG.as_ref().unwrap()
            },
            Some(v) => v
        }
    }
}


#[derive(Deserialize, Debug)]
pub struct Config {
    pub user: User
}

impl Config {
    fn default() -> Self {
        let cv = fs::read("conf.toml").expect("配置文件读取异常!");
        toml::from_slice(cv.as_slice()).expect("配置文件解析异常!")
    }
}

