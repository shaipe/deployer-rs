//! copyright © ecdata.cn 2021 - present
//! 日志查看项目配置信息
//! created by shaipe 20210509

use std::{fs::read_dir, path::{Path, PathBuf}};
use serde::{Deserialize, Serialize};

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
    // 日志的文件扩展名
    pub extension: String,
    // 日志文件
    // pub logs: Vec<PathBuf>,
}

impl Log {
    pub fn get_files(&self) -> Vec<FileInfo> {
        vec![]
    }
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileInfo {
    pub name: String,
    pub modify_time: String,
    pub created_time: String,
    pub size: String,
    pub path: String,
}



/// 遍历所有目录
/// @param1: 目录路径
/// @param2: 过滤后缀名
fn walk_dir(dir_path: &Path, ext: &str) -> Vec<PathBuf> {
    let mut res = Vec::new();
    if dir_path.exists() {
        for entry in read_dir(dir_path).unwrap() {
            let path = entry.unwrap().path();
            if ext.len() > 0 {
                // 获取并判断后缀是否相等
                if let Some(ex) = path.extension() {
                    if let Some(ex_str) = ex.to_str() {
                        if ex_str.to_lowercase() == ext.to_lowercase() {
                            res.push(path.clone());
                        }
                    }
                }
            } else {
                // 如果没有给定后缀名则全部加入
                res.push(path.clone());
            }

            if path.is_dir() {
                let sub_dirs = walk_dir(path.as_path(), ext);
                res.extend(sub_dirs);
            }
        }
    }
    res
}