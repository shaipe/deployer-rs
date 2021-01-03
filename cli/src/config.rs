//! copyright © shaipe 2021 - present
//! 配置工具
//! create by shaipe 20210102

use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::result::Result;
use yaml_rust::yaml;

/// 配置信息
#[derive(Debug, Clone)]
pub struct Config {
    pub server: Server,
    pub local: Local,
    pub commands: Vec<String>,
}

/// 服务器信息
#[derive(Debug, Clone)]
pub struct Server {
    // 服务器ip地址
    pub ip: String,
    // 服务端口
    pub port: u16,
    // 用户名
    pub username: String,
    // 密码
    pub password: String,
    // 上传的路径
    pub path: String,
}

/// 本地信息配置
#[derive(Debug, Clone)]
pub struct Local {
    pub home: String,
}

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

        // println!("{:?}", yaml_doc);
        // get server value
        let server = yaml_doc["server"].clone();
        let local = yaml_doc["local"].clone();
        let cmds = yaml_doc["commands"].clone();

        println!("server: {:?}", server);

        println!("local: {:?}", local);

        println!("commands: {:?}", cmds);

        Ok(Config {
            server: Server {
                ip: "0.0.0.0".to_owned(),
                port: 3000,
                username: "root".to_owned(),
                password: "".to_owned(),
                path: "/".to_owned(),
            },
            local: Local {
                home: "/".to_owned(),
            },
            commands: vec![],
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
                username: "root".to_owned(),
                password: "".to_owned(),
                path: "/".to_owned(),
            },
            local: Local {
                home: "/".to_owned(),
            },
            commands: vec![],
        }
    }
}
