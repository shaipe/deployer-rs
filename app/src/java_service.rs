//! copyright © ecdata.cn 2021 - present
//! Java语言的服务启动安装
//! created by shaipe 20210427

use std::{
    fs::{create_dir_all, OpenOptions},
    os::unix::prelude::OpenOptionsExt,
};
use std::{io::Write, path::Path};

/**
    1. 创建服务启动脚本,因为直接使用服务启动jar时不能记录日志
    2. 默认日志目录为当前应用的 logs/
*/
// use super::cmd::run_cmd;
// use tube_error::Result;

pub struct JavaService {}

impl JavaService {
    /// 安装服务启动脚本应用
    pub fn service_shell(shell_path: &str, name: &str, exec_cmd: &str) -> Vec<String> {
        let res = Vec::new();
        let shell_content = format!(
            r#"#!/bin/bash

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
        // 把执行代码写入启动文件中
        write_exec_file(shell_path, &shell_content.as_bytes());

        res
    }
}

/// 把数据写入到文件中
fn write_exec_file(file_path: &str, content: &[u8]) {
    // 判断目录是否存在,不存在即创建目录
    let p = Path::new(&file_path);
    let f_dir = p.parent().unwrap();
    if !f_dir.exists() {
        let _ = create_dir_all(f_dir);
    }

    // 创建文件写入并可执行的对象

    let mut file = OpenOptions::new()
        .create(true)
        .write(true)
        .mode(0o770)
        .open(file_path)
        .unwrap();

    // 将数据流写入文件
    match file.write_all(content) {
        Err(why) => {
            panic!("couldn't write to : {}", why)
        }
        Ok(_) => {
            println!("successfully wrote to {}", file_path)
        }
    };
}
