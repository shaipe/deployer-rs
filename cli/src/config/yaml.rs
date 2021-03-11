//! copyright © shaipe 2021 - present
//! 使用yml格式的配置中加载
//! create by shaipe 20210125

use super::Task;
use micro_app::{App, Docker, Remote, Service};
use std::collections::HashMap;
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
    let version = doc["version"].get_string("");
    let desc = doc["description"].get_string("");
    let app = load_app(&name, &symbol, &desc, &doc["app"]);
    let replaces: HashMap<&str, String> = [
        ("$symbol", symbol.clone()),
        ("$name", name.clone()),
        ("$version", version.clone()),
    ]
    .iter()
    .cloned()
    .collect();
    let app_type = doc["type"].get_string("files");
    // 类型为服务时才加载服务信息
    let srv = if app_type == "service" {
        load_service(&app, &doc["service"])
    } else {
        None
    };

    // 加载容器类型
    let dock = if app_type == "docker" {
        load_docker(&doc["docker"])
    } else {
        None
    };

    Task {
        name: name.clone(),
        symbol: symbol.clone(),
        version: version.clone(),
        description: desc.clone(),
        app_type: app_type,
        app: app.clone(),
        remote: load_remote(&app, &doc["remote"]),
        service: srv,
        docker: dock,
        start: vec_var_replace(doc["start"].get_vec(), replaces.clone()),
        end: vec_var_replace(doc["end"].get_vec(), replaces.clone()),
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
pub fn load_service(app: &App, doc: &Yaml) -> Option<Service> {
    if doc.is_null() {
        None
    } else {
        let mut srv_name = app.name.clone();
        if app.symbol.len() > 0 {
            srv_name = format!("{}_{}", app.symbol, app.name);
        }

        let workdir = format!(
            "{}/{}/{}",
            doc["workdir"].get_string(""),
            app.symbol,
            app.name
        );

        let replaces: HashMap<&str, String> = [
            ("$workdir", workdir.clone()),
            ("$symbol", app.symbol.clone()),
            ("$name", app.name.clone()),
        ]
        .iter()
        .cloned()
        .collect();

        // 执行启动程序
        let mut exec = var_replace(doc["exec"].get_string(""), replaces.clone());
        if exec.len() < 1 {
            exec = format!("{}/{}", workdir, app.app_name());
        }
        // 处理服务执行参数
        let arg = var_replace(doc["args"].get_string(""), replaces.clone());
        Some(Service {
            name: srv_name,
            exec: exec,
            args: if arg.len() > 0 { Some(arg) } else { None },
            workdir: workdir,
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
        code_dir: doc["code_dir"].get_string("."),
    }
}

/// 加载远程处理配置
pub fn load_remote(app: &App, doc: &Yaml) -> Option<Remote> {
    if doc.is_null() {
        return None;
    }
    let replaces: HashMap<&str, String> =
        [("$symbol", app.symbol.clone()), ("$name", app.name.clone())]
            .iter()
            .cloned()
            .collect();

    Some(Remote {
        server: doc["server"].get_string("127.0.0.1"),
        port: if let Some(p) = doc["port"].as_i64() {
            p as u16
        } else {
            3000
        },
        workdir: doc["workdir"].get_string("."),
        start: vec_var_replace(doc["start"].get_vec(), replaces.clone()),
        end: vec_var_replace(doc["end"].get_vec(), replaces.clone()),
    })
}

/// 对变量进行替换
fn var_replace(mut sources: String, replaces: HashMap<&str, String>) -> String {
    if sources.len() < 1 {
        return sources;
    }
    for (k, v) in replaces {
        sources = sources.replace(k, &v);
    }
    sources
}

/// 替换数组中的关键字变量
fn vec_var_replace(sources: Vec<String>, replaces: HashMap<&str, String>) -> Vec<String> {
    let mut res = Vec::new();
    for mut s in sources {
        for (k, v) in replaces.clone() {
            s = s.replace(k, &v);
        }
        res.push(s);
    }
    res
}
