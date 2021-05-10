//! copyright © ecdata.cn 2021 - present
//! 日志查看项目
//! created by shaipe 20210509

use std::{
    fs::read_dir,
    path::{Path, PathBuf},
};

#[macro_use]
extern crate lazy_static;

mod config;

pub use config::{get_config, set_config, Config};

/// 获取日志文件列表
pub fn get_log_files(name: &str) -> Vec<PathBuf> {
    if let Some(conf) = get_config() {
        if let Some(log) = conf.get_log(name){
            // log.get_files
        }
    }
    vec![]
}



#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
