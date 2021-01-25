//! copyright © shaipe 2021 - present
//! 使用yml格式的配置中加载
//! create by shaipe 20210125

use super::{Remote, Task};
use micro_app::App;
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
pub fn load_tasks(doc: &Yaml) -> Vec<Task> {
    let mut res: Vec<Task> = Vec::new();
    if doc.is_array() {
        if let Some(ts) = doc.as_vec() {
            for doc_task in ts.iter() {
                let task = load_task(doc_task);
                res.push(task);
            }
        }
    }
    res
}

/// 加载任务
pub fn load_task(doc: &Yaml) -> Task {
    Task {
        name: doc["name"].get_string(""),
        app: load_app(&doc["app"]),
        remote: load_remote(&doc["remote"]),
        start: doc["start"].get_vec(),
        end: doc["end"].get_vec(),
    }
}

/// 加载应用
pub fn load_app(doc: &Yaml) -> App {
    App {
        symbol: doc["symbol"].get_string(""),
        name: doc["name"].get_string(""),
        description: doc["description"].get_string(""),
        port: 3000,
        exec_start: doc["exec_start"].get_string(""),
        version: doc["version"].get_string("0.1.0"),
        lang: doc["lang"].get_string("rust"),
        workdir: doc["workdir"].get_string("./"),
        is_service: doc["is_service"].get_bool(),
        status: 1,
    }
}

/// 加载远程处理配置
pub fn load_remote(doc: &Yaml) -> Remote {
    Remote {
        server: doc["server"].get_string("127.0.0.1"),
        port: if let Some(p) = doc["port"].as_i64() {
            p as u16
        } else {
            3000
        },
        workdir: doc["workdir"].get_string("./"),
        start: doc["start"].get_vec(),
        end: doc["end"].get_vec(),
    }
}
