//! copyright © ecdata.cn 2021 - present
//! 日志查看项目配置
//! created by shaipe 20210509

mod log;
mod yaml;

pub use log::Log;
use std::fs::File;
use std::{collections::HashMap, error::Error, io::Read, sync::Mutex};
use yaml::{load_logs, YamlImpl};

#[derive(Debug, Clone)]
pub struct Config {
    version: String,
    logs: Vec<Log>,
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
            logs: load_logs(&yaml_doc["logs"]),
        })
    }

    /// 根据名称获取任务
    pub fn get_log(&self, name: &str) -> Option<Log> {
        if name.len() > 0 {
            for t in self.logs.clone() {
                if t.name == name {
                    return Some(t);
                }
            }
        }
        None
    }
}

// 默认加载静态全局
lazy_static! {
    pub static ref CONFIG_CACHES: Mutex<HashMap<String, Config>> = Mutex::new(HashMap::new());
}

/// 将创建的websocket服务器存入缓存中
#[allow(dead_code)]
pub fn set_config(conf: Config) {
    CONFIG_CACHES
        .lock()
        .unwrap()
        .insert("server_logs_config".to_owned(), conf);
}

/// 获取websocket服务器
#[allow(dead_code)]
pub fn get_config() -> Option<Config> {
    let cache = CONFIG_CACHES.lock().unwrap();
    let v = match cache.get("server_logs_config") {
        Some(val) => Some(val.clone()),
        _ => None,
    };
    return v;
}
