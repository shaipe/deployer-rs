//! copyright © shaipe 2021 - present
//! 任务信息
//! create by shaipe 20210124

use micro_app::App;
use super::Remote;
use serde::{Deserialize, Serialize};

/// 任务结构体
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Task {
    // 任务名称
    pub name: String,
    // 应用信息
    pub app: App,
    // 远程信息
    pub remote: Remote,
    // 开始执行
    pub start: Vec<String>,
    // 结束执行
    pub end: Vec<String>,
}

