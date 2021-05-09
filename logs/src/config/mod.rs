//! copyright © ecdata.cn 2021 - present
//! 日志查看项目配置
//! created by shaipe 20210509

mod log;
mod yaml;

pub use log::Log;



pub struct Config {
    logs: Vec<Log>,
}

impl Config {
    fn new(conf_path: &str) -> Config {
        Config {
            logs: Vec::new(),
        }
    }
}
