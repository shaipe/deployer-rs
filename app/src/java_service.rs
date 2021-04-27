//! copyright © ecdata.cn 2021 - present
//! Java语言的服务启动安装
//! created by shaipe 20210427

/**
    1. 创建服务启动脚本,因为直接使用服务启动jar时不能记录日志
    2. 默认日志目录为当前应用的 logs/
*/
use super::cmd::run_cmd;
// use tube_error::Result;

pub struct JavaService {}

impl JavaService {
    /// 安装服务启动脚本应用
    pub fn service_shell(shell_path: &str, name: &str, exec_cmd: &str) -> Vec<String> {
        let mut res = Vec::new();
        let shell_content = format!(
            r#"
#!/bin/bash

# 判断日志目录是否存，不存在创建目录
if [ ! -d "logs/" ];then
    mkdir logs
fi

# 启动应用，并给定输出日志目录
{exec} > logs/{name}-$(date +%Y%m%d%H%M%S).log
"#,
            exec = exec_cmd,
            name = name
        );
        // println!("{}\n{}", path, srv_content);
        // 把文件写入服务
        tube::fs::write_file(shell_path, &shell_content.as_bytes());

        if let Ok(_r) = run_cmd(shell_path, "", true) {
            res.push("chmod permission successfully!".to_owned());
        }

        res
    }
}
