//! copyright © ecdata.cn 2021 - present
//! 使用yml格式的配置中加载
//! create by shaipe 20210125

use std::path::Path;

use super::Log;
use yaml_rust::Yaml;

/// 基于yaml扩展接口
pub trait YamlImpl {
    fn get_string(&self, def: &str) -> String;

    fn get_bool(&self) -> bool;

    fn get_vec(&self) -> Vec<String>;
}

impl YamlImpl for Yaml {
    /// 获取字符串
    fn get_string(&self, def: &str) -> String {
        if let Some(v) = self.as_str() {
            v.to_owned()
        } else {
            def.to_owned()
        }
    }

    /// 获取bool型
    fn get_bool(&self) -> bool {
        println!("{:?}", self);
        if let Some(v) = self.as_bool() {
            v
        } else {
            false
        }
    }

    /// 获取数组
    fn get_vec(&self) -> Vec<String> {
        if let Some(dir) = self.as_vec() {
            dir.iter()
                .map(|x| {
                    if let Some(y) = x.as_str() {
                        y.to_owned()
                    } else {
                        "".to_owned()
                    }
                })
                .collect()
        } else {
            vec![]
        }
    }
}

/// 加载任务集
pub fn load_logs(doc: &Yaml) -> Vec<Log> {
    let mut res: Vec<Log> = Vec::new();
    if doc.is_array() {
        if let Some(ts) = doc.as_vec() {
            for doc_log in ts.iter() {
                let log = load_log(doc_log);
                res.push(log);
            }
        }
    }
    res
}

/// 加载任务
pub fn load_log(doc: &Yaml) -> Log {
    let name = doc["name"].get_string("");
    let title = doc["title"].get_string("");
    let server = doc["server"].get_string("");
    let dir = doc["dir"].get_string("");
    let extension = doc["extension"].get_string("log");

    Log {
        name: name,
        title: title,
        server: server,
        dir: dir.clone(),
        extension: extension,
        // logs: tube::fs::walk_dir(Path::new(&dir)),
    }
}
