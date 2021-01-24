//! copyright © shaipe 2021 - present
//! 远程服务器配置信息
//! create by shaipe 20210124

use serde::{Deserialize, Serialize};
use yaml_rust::Yaml;

/// 服务器信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Remote {
    // 服务器地址
    pub server: String,
    // 服务端口
    pub port: u16,
    // 工作目录
    pub workdir: String,
    // 远程开始执行的命令
    pub start: Vec<String>,
    // 远程完成后执行命令
    pub end: Vec<String>,
}

impl Remote {
    /// 从yaml配置中加载远程信息
    pub fn load_yaml(doc: &Yaml) -> Self {
        use super::YamlImpl;
        Remote {
            server: doc["server"].get_string("127.0.0.1"),
            port: if let Some(p) = doc["port"].as_i64() {
                p as u16
            } else {
                3000
            },
            workdir: doc["workdir"].get_string("./"),
            start: doc["start"].get_vec(),
            end: doc["end"].get_vec(),
        }
    }

    /// 根据配置获url
    pub fn get_url(&self) -> String {
        // 判断处理端口问题
        let port_str = if self.port > 0 {
            format!(":{}", self.port)
        } else {
            "".to_owned()
        };

        format!("http://{}{}", self.server, port_str)
    }
}
