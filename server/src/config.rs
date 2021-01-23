//! copyright © shaipe 2021 - present
//! 配置工具
//! create by shaipe 20210102

use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;
use std::sync::Mutex;
use tube_error::Result;
use yaml_rust::yaml;

/// 配置信息
#[derive(Debug, Clone)]
pub struct Config {
    // 服务配置信息
    pub server: Server,
    // 工作目录
    pub workdir: String,
}

/// 服务器信息
#[derive(Clone, Debug)]
pub struct Server {
    // IP地址
    pub ip: String,
    // 端口
    pub port: u64,
}

impl Config {
    /// 获取一个配置的新实例，并从指定的配置中加载配置信息
    pub fn new(conf_path: &str) -> Result<Config> {
        // open file
        let mut f = match File::open(conf_path) {
            Ok(f) => f,
            Err(e) => {
                return Err(error! {
                    code: 4004,
                    msg: format!("{}", e)
                });
            }
        };
        let mut s = String::new();
        f.read_to_string(&mut s).unwrap(); // read file content to s
                                           // load string to yaml loader
        let docs = yaml::YamlLoader::load_from_str(&s).unwrap();
        // get first yaml hash doc
        let yaml_doc = &docs[0];
        // get server value
        let server = yaml_doc["server"].clone();

        let cnf = Config {
            server: Server {
                ip: server["ip"].as_str().unwrap().to_owned(),
                port: server["port"].as_i64().unwrap() as u64,
            },
            workdir: if let Some(dir) = yaml_doc["workdir"].as_str() {
                dir.to_owned()
            } else {
                "./".to_owned()
            },
        };
        set_config(cnf.clone());
        Ok(cnf)
    }
}

/// 默认输出
impl std::default::Default for Config {
    fn default() -> Self {
        Config {
            server: Server {
                ip: "0.0.0.0".to_owned(),
                port: 3000,
            },
            workdir: "./".to_owned(),
        }
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
        .insert("server_config".to_owned(), conf);
}

/// 获取websocket服务器
#[allow(dead_code)]
pub fn get_config() -> Option<Config> {
    let cache = CONFIG_CACHES.lock().unwrap();
    let v = match cache.get("server_config") {
        Some(val) => Some(val.clone()),
        _ => None,
    };
    return v;
}
