//! copyright © shaipe 2021 - present
//! 配置工具
//! create by shaipe 20210102

use std::fs::File;
use std::io::prelude::*;
use tube_error::Result;
use yaml_rust::yaml;

/// 配置信息
#[derive(Debug, Clone)]
pub struct Config {
    pub server: Server,
}

/// 服务器信息
#[derive(Clone, Debug)]
pub struct Server {
    pub ip: String,
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

        Ok(Config {
            server: Server {
                ip: server["ip"].as_str().unwrap().to_owned(),
                port: server["port"].as_i64().unwrap() as u64,
            },
        })
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
        }
    }
}
