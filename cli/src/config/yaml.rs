//! copyright © shaipe 2021 - present
//! 使用yml格式的配置中加载
//! create by shaipe 20210125

use super::Task;
use micro_app::{App, Docker, Remote, Service};
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
    let name = doc["name"].get_string("");
    let symbol = doc["symbol"].get_string("");
    let desc = doc["description"].get_string("");
    Task {
        name: name.clone(),
        symbol: symbol.clone(),
        description: desc.clone(),
        app_type: doc["type"].get_string(""),
        app: load_app(&name, &symbol, &desc, &doc["app"]),
        remote: load_remote(&doc["remote"]),
        service: load_service(&name, &symbol, &doc["service"]),
        docker: load_docker(&doc["docker"]),
        start: doc["start"].get_vec(),
        end: doc["end"].get_vec(),
    }
}

/// 加载容器配置
pub fn load_docker(doc: &Yaml) -> Option<Docker> {
    if doc.is_null() {
        return None;
    }
    None
}

/// 加载服务配置
pub fn load_service(name: &str, symbol: &str, doc: &Yaml) -> Option<Service> {
    if doc.is_null() {
        None
    } else {
        let mut srv_name = name.to_owned();
        if symbol.len() > 0 {
            srv_name = format!("{}_{}", symbol, name);
        }
        let arg = doc["args"].get_string("");

        Some(Service {
            name: srv_name,
            exec: doc["workdir"].get_string(""),
            args: if arg.len() > 0 { Some(arg) } else { None },
            workdir: doc["workdir"].get_string(""),
            timeout: 60,
        })
    }
}

/// 加载应用
pub fn load_app(name: &str, symbol: &str, desc: &str, doc: &Yaml) -> App {
    App {
        symbol: symbol.to_owned(),
        name: name.to_owned(),
        description: desc.to_owned(),
        version: doc["version"].get_string("0.1.0"),
        lang: doc["lang"].get_string("rust"),
        code_dir: doc["workdir"].get_string("./"),
    }
}

/// 加载远程处理配置
pub fn load_remote(doc: &Yaml) -> Option<Remote> {
    if doc.is_null() {
        return None;
    }
    Some(Remote {
        server: doc["server"].get_string("127.0.0.1"),
        port: if let Some(p) = doc["port"].as_i64() {
            p as u16
        } else {
            3000
        },
        workdir: doc["workdir"].get_string("./"),
        start: doc["start"].get_vec(),
        end: doc["end"].get_vec(),
    })
}
