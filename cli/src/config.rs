//! copyright © shaipe 2021 - present
//! 配置工具
//! create by shaipe 20210102

use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::result::Result;
use yaml_rust::yaml;
// use std::path::Path;

/// 配置信息
#[derive(Debug, Clone)]
pub struct Config {
    // 本地
    pub local: Local,
    // 服务器的配置信息
    pub remote: Remote,
}

/// 服务器信息
#[derive(Debug, Clone)]
pub struct Remote {
    // 服务器ip地址
    pub uri: String,
    // // 服务端口
    // pub port: u16,
    // // 用户名
    // pub username: String,
    // // 密码
    // pub password: String,
    // // 上传的路径
    // pub path: String,
    // 工作目录
    pub workdir: String,
    // 远程需要执行的命令
    pub commands: Vec<String>,
}

/// 本地信息配置
#[derive(Debug, Clone)]
pub struct Local {
    // 工作目录
    pub workdir: String,

    // 本地开始执行的命令
    pub start_cmd: Vec<String>,

    // 完成后执行命令
    pub end_cmd: Vec<String>,

    // 上传的url地址
    pub upload_url: String,

    // 需要上传的文件路径
    pub upload_file: String,
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
        // get remote value
        let remote = yaml_doc["remote"].clone();
        let local = yaml_doc["local"].clone();

        Ok(Config {
            remote: Remote {
                uri: if let Some(url) = remote["uri"].as_str() {
                    url.to_owned()
                } else {
                    "http://127.0.0.1/cmd".to_owned()
                },
                workdir: if let Some(dir) = remote["workdir"].as_str() {
                    dir.to_owned()
                } else {
                    "./".to_owned()
                },
                commands: if let Some(dir) = remote["commands"].as_vec() {
                    dir.iter().map(|x|if let Some(y) = x.as_str(){
                        y.to_owned()
                    }else{
                        "".to_owned()
                    }).collect()
                } else {
                    vec![]
                },
            },
            local: Local {
                workdir: if let Some(dir) = local["workdir"].as_str() {
                    dir.to_owned()
                } else {
                    "./".to_owned()
                },
                start_cmd: if let Some(cmds) = local["commands"]["start"].as_vec() {
                    cmds.iter().map(|x|if let Some(y) = x.as_str(){
                        y.to_owned()
                    }else{
                        "".to_owned()
                    }).collect()
                } else {
                    vec![]
                },
                end_cmd: if let Some(cmds) = local["commands"]["end"].as_vec() {
                    cmds.iter().map(|x|if let Some(y) = x.as_str(){
                        y.to_owned()
                    }else{
                        "".to_owned()
                    }).collect()
                } else {
                    vec![]
                },
                upload_url: if let Some(dir) = local["upload"]["uri"].as_str() {
                    dir.to_owned()
                } else {
                    "http://127.0.0.1:8080/upload".to_owned()
                },
                upload_file: if let Some(dir) = local["upload"]["file"].as_str() {
                    dir.to_owned()
                } else {
                    "name.zip".to_owned()
                },
            },
        })
    }
}

/// 默认输出
impl std::default::Default for Config {
    fn default() -> Self {
        Config {
            remote: Remote {
                uri: "http://127.0.0.1/cmd".to_owned(),
                workdir: "./".to_owned(),
                commands: vec![],
            },
            local: Local {
                workdir: "./".to_owned(),
                start_cmd: vec![],
                end_cmd: vec![],
                upload_url: "http://127.0.0.1/upload".to_owned(),
                upload_file: "".to_owned(),
            },
        }
    }
}
