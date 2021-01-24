//! copyright © shaipe 2021 - present
//! 配置工具
//! create by shaipe 20210102

use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::result::Result;
use yaml_rust::{yaml, Yaml};
// use std::path::Path;
// use micro_app::App;

mod app;
mod remote;
mod yaml_impl;
pub use app::App;
pub use remote::Remote;
pub use yaml_impl::YamlImpl;

/// 配置信息
#[derive(Debug, Clone)]
pub struct Config {
    pub version: String,
    // 应用集
    pub apps: Vec<App>,
}

// /// 本地信息配置
// #[derive(Debug, Clone)]
// pub struct Local {
//     // 工作目录
//     pub workdir: String,

//     // 本地开始执行的命令
//     pub start_cmd: Vec<String>,

//     // 完成后执行命令
//     pub end_cmd: Vec<String>,

//     // 上传的url地址
//     pub upload_url: String,

//     // 需要上传的文件路径
//     pub upload_file: String,
// }

impl Config {
    pub fn new(conf_path: &str) -> Result<Config, Box<dyn Error>> {
        // open file
        let mut f = match File::open(conf_path) {
            Ok(f) => f,
            Err(e) => {
                panic!("no such file {} exception: {}", conf_path, e);
            }
        };
        let mut s = String::new();
        f.read_to_string(&mut s).unwrap(); // read file content to s
                                           // load string to yaml loader
        let docs = yaml::YamlLoader::load_from_str(&s).unwrap();
        // get first yaml hash doc
        let yaml_doc = &docs[0];

        Ok(Config {
            version: yaml_doc["version"].get_string("0.1.0"),
            apps: load_apps(&yaml_doc["apps"]),
        })
    }

    /// 根据名称获取应用
    pub fn get_app(&self, name: &str) -> Option<App> {
        if name.len() > 0 {
            for app in self.apps.clone() {
                if app.name == name {
                    return Some(app);
                }
            }
        }
        None
    }
}

/// 从配置中读取应用
fn load_apps(doc_apps: &Yaml) -> Vec<App> {
    let mut res: Vec<App> = Vec::new();
    if doc_apps.is_array() {
        if let Some(apps) = doc_apps.as_vec() {
            for doc_app in apps.iter() {
                let app = App::load_yaml(doc_app);
                res.push(app);
            }
        }
    }
    res
}
