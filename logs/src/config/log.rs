//! copyright © ecdata.cn 2021 - present
//! 日志查看项目配置信息
//! created by shaipe 20210509

#[derive(Debug, Clone)]
pub struct Log {
    // 名称
    pub name: String,
    // 服务器
    pub server: String,
    // 标题
    pub title: String,
    // 存放目录
    pub dir: String,
    // 日志文件
    pub logs: Vec<String>,
}
