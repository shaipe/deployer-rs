//! copyright © shaipe 2021 - present
//! 远程服务器配置信息
//! create by shaipe 20210124

use serde::{Deserialize, Serialize};

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
