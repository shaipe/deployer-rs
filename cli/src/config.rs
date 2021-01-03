//! copyright © shaipe 2021 - present
//! 配置工具
//! create by shaipe 20210102

use std::fs::File;
use std::io::prelude::*;
use yaml_rust::yaml;
use std::error::Error;
use std::result::Result;

#[derive(Debug, Clone)]
pub struct Config {
    pub server: String,
}

impl Config {
    pub fn new(conf_path: &str) -> Result<Config, Box<dyn Error>> {
        
        // open file 
        let mut f = match File::open(conf_path){
            Ok(f) => f,
            Err(e) => {
                panic!("no such file {} exception: {}", conf_path, e);
            }
        };
        let mut s = String::new();
        f.read_to_string(&mut s).unwrap();  // read file content to s
        // load string to yaml loader
        let docs = yaml::YamlLoader::load_from_str(&s).unwrap();
        // get first yaml hash doc
        let yaml_doc = &docs[0];

        println!("{:?}", yaml_doc);
        // get server value
        let server = yaml_doc["server"]["ip"].clone().into_string().unwrap();
                
        Ok(Config {
            server
        })
    }
}

