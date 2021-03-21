//! copyright © shaipe 2021 - present
//! 微服务应用管理
//! create by shaipe 20210123

// use super::service;
use serde::{Deserialize, Serialize};
// use tube_error::Result;

/// 应用
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct App {
    // 应用项目代号
    pub symbol: String,
    // 微服务名称
    pub name: String,
    // 标题
    pub description: String,
    // 代码目录
    pub code_dir: String,
    // 当前版本
    pub version: String,
    // 应用开发语言
    pub lang: String,
    // 目录操作系统
    pub os: String,
}

impl App {
    /// 新建应用对象
    pub fn new(base_dir: &str, symbol: &str, name: &str) -> App {
        App {
            symbol: symbol.to_owned(),
            name: name.to_owned(),
            description: "".to_owned(),
            code_dir: format!(
                "{workdir}/{symbol}/apps/{name}",
                workdir = base_dir,
                symbol = symbol,
                name = name
            ),
            version: "0.1.0".to_owned(),
            lang: "java".to_owned(),
            os: "linux".to_owned(),
        }
    }

    

    // /// 安装服务
    // pub fn install_service(&self) -> Result<bool> {
    //     let srv_name = format!("{}_{}", self.symbol, self.name);
    //     let mut cmd = self.exec_start.clone();
    //     if let Some(arg) = self.exec_arg.clone() {
    //         // 替换系统中的变量
    //         cmd = format!(
    //             "{} {}",
    //             self.exec_start,
    //             arg.replace("$workdir", &self.workdir)
    //         );
    //     }

    //     // 安装服务
    //     service::Service::new(&self.workdir, &srv_name, &cmd, 60).install()
    // }

    /// 获取jar应用名
    pub fn app_name(&self) -> String {
        if self.symbol.len() > 0 {
            format!("{}_{}{}", self.symbol, self.name, self.get_ext())
        } else {
            format!("{}{}", self.name, self.get_ext())
        }
    }

    /// 获取应用包的扩展名
    fn get_ext(&self) -> &'static str {
        if &self.lang == "java" {
            ".jar"
        } else {
            // 此处不应该用target_os,这里是指上传时的平台
            if &self.os == "windows" {
                ".exe"
            } else if &self.os == "macos" {
                ".app"
            } else {
                ""
            }
        }
    }

    
}
