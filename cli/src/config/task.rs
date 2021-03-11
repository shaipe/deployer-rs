//! copyright © shaipe 2021 - present
//! 任务信息
//! create by shaipe 20210124

use micro_app::{App, Docker, Remote, Service};
use serde::{Deserialize, Serialize};

/// 任务结构体
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Task {
    // 任务名称
    pub name: String,
    // 应用代号
    pub symbol: String,
    // 应用版本
    pub version: String,
    // 应用描述
    pub description: String,
    // 应用类型
    #[serde(rename = "appType")]
    pub app_type: String,
    // 应用信息
    pub app: App,
    // 服务
    pub service: Option<Service>,
    // 容器信息
    pub docker: Option<Docker>,
    // 远程信息
    pub remote: Option<Remote>,
    // 开始执行
    pub start: Vec<String>,
    // 结束执行
    pub end: Vec<String>,
    
}
