//! copyright © shaipe 2021 - present
//! 应用配置信息
//! create by shaipe 20210124

use super::Remote;
use serde::{Deserialize, Serialize};
use yaml_rust::Yaml;

/// 应用
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct App {
    // 应用项目代号
    pub symbol: String,
    // 微服务名称
    pub name: String,
    // 标题
    pub description: String,
    // 应用工作目录
    pub workdir: String,
    // 端口
    // 当前版本
    pub version: String,
    // 应用开发语言
    pub lang: String,
    // 应用对应用的远程信息
    pub remote: Remote,
    // 安装或更新开始执行的命令
    pub start: Vec<String>,
    // 安装或更新远程执行完成后执行的命令配置
    pub end: Vec<String>,
}

impl App {
    /// 从yaml配置中加载app信息
    pub fn load_yaml(doc: &Yaml) -> Self {
        use super::YamlImpl;
        App {
            symbol: doc["symbol"].get_string(""),
            name: doc["name"].get_string(""),
            description: doc["description"].get_string(""),
            version: doc["version"].get_string("0.1.0"),
            lang: doc["lang"].get_string("rust"),
            workdir: doc["workdir"].get_string("./"),
            remote: Remote::load_yaml(&doc["remote"]),
            start: doc["start"].get_vec(),
            end: doc["end"].get_vec(),
        }
    }
}
