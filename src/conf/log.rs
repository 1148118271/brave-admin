use serde::Deserialize;


#[derive(Deserialize, Debug)]
pub struct Log {
    pub log_file_path: String
}

impl Log {
    pub fn init() {
        let conf = super::config::default();
        fern::Dispatch::new()
            // Perform allocation-free log formatting
            .format(|out, message, record| {
                out.finish(format_args!(
                    "{}[{}][{}] {}",
                    chrono::Local::now().format("[%Y-%m-%d %H:%M:%S]"),
                    record.target(),
                    record.level(),
                    message
                ))
            })
            // Add blanket level filter -
            .level(log::LevelFilter::Debug)
            // - and per-module overrides
            .level_for("hyper", log::LevelFilter::Debug)
            // Output to stdout, files, and other Dispatch configurations
            .chain(std::io::stdout())
            .chain(fern::log_file(&conf.log.log_file_path).expect("获取日志文件输出路径异常"))
            // Apply globally
            .apply().expect("日志启动异常")
    }
}