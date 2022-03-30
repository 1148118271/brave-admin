use std::fs::File;
use serde::Deserialize;
use log::LevelFilter;
use simplelog::{ColorChoice, CombinedLogger, Config, TerminalMode, TermLogger, WriteLogger};

#[derive(Deserialize, Debug)]
pub struct Log {
    pub log_file_path: String
}

impl Log {
    pub fn init() {
        let conf = super::config::default();
        CombinedLogger::init(
            vec![
                TermLogger::new(LevelFilter::Debug, Config::default(),
                                TerminalMode::Mixed, ColorChoice::Auto),
                WriteLogger::new(LevelFilter::Info, Config::default(),
                                 File::create(&conf.log.log_file_path).expect("生成日志文件异常")),
            ]
        ).expect("初始化日志异常");
    }
}