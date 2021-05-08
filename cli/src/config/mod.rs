//! copyright © ecdata.cn 2021 - present
//! 配置工具
//! create by shaipe 20210102

use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::result::Result;

mod task;
mod yaml;
pub(crate) use task::Task;
use yaml::{load_tasks, YamlImpl};

/// 配置信息
#[derive(Debug, Clone)]
pub struct Config {
    pub version: String,
    // 应用集
    pub tasks: Vec<Task>,
}

impl Config {
    pub fn new(conf_path: &str) -> Result<Config, Box<dyn Error>> {
        // println!("config path: {}", conf_path);
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

        // println!("{}", s);
        let docs = yaml_rust::yaml::YamlLoader::load_from_str(&s).unwrap();
        // get first yaml hash doc
        let yaml_doc = &docs[0];

        Ok(Config {
            version: yaml_doc["version"].get_string("0.1.0"),
            tasks: load_tasks(&yaml_doc["tasks"]),
        })
    }

    /// 根据名称获取任务
    pub fn get_task(&self, name: &str) -> Option<Task> {
        if name.len() > 0 {
            for t in self.tasks.clone() {
                if t.name == name {
                    return Some(t);
                }
            }
        }
        None
    }
}

// /// 从配置中读取应用
// fn load_apps(doc_apps: &Yaml) -> Vec<App> {
//     let mut res: Vec<App> = Vec::new();
//     if doc_apps.is_array() {
//         if let Some(apps) = doc_apps.as_vec() {
//             for doc_app in apps.iter() {
//                 let app = App::load_yaml(doc_app);
//                 res.push(app);
//             }
//         }
//     }
//     res
// }
