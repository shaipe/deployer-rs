//! copyright © shaipe 2021 - present
//! 对Yaml操作进行扩展
//! create by shaipe 20210124

use yaml_rust::Yaml;

pub trait YamlImpl {
    fn get_string(&self, def: &str) -> String;

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