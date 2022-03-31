use std::fs;
use serde::Deserialize;
use super::log::Log;
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
    pub user: User,
    pub log: Log,
    pub database: Database
}

#[derive(Deserialize, Debug)]
pub struct Database {
    pub url: String
}

impl Config {
    fn default() -> Self {
        let cv = fs::read("conf.toml").expect("配置文件读取异常!");
        toml::from_slice(cv.as_slice()).expect("配置文件解析异常!")
    }
}

