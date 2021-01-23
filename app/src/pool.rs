//! copyright © shaipe 2021 - present
//! 微服务应用池管理
//! create by shaipe 202101023

use serde::{Deserialize, Serialize};
use tube_error::Result;

/// 应用池
#[derive(Debug, Clone, Serialize, Deserialize)]
struct Pool {
    // 应用池名称
    name: String,
    // 应用池工作目录
    workdir: String,
    // 应用集
    apps: Vec<App>,
}

impl Pool {
    /// 新建应用池对象
    pub fn new(name: String, workdir: String) -> Pool {
        Pool {
            name: name,
            workdir: workdir,
            apps: vec![],
        }
    }

    /// 初始化加载应用池信息
    pub fn load(workdir: &str, name: &str) -> Result<Pool> {
        use std::fs::create_dir_all;
        use std::fs::File;
        use std::io::BufReader;
        use std::path::Path;

        // 判断工作目录是否存在
        let dir_path = Path::new(workdir);
        if !dir_path.exists() {
            // 如果目录不存在就创建目录
            match create_dir_all(workdir) {
                Ok(_) => {}
                Err(err) => return Err(error!(format!("{:?}", err))),
            }
        }
        // 给定应用池文件
        let pool_file = format!("{}/pool.json", workdir);
        let pool_path = Path::new(&pool_file);
        if pool_path.exists() {
            // Open the file in read-only mode with buffer.
            let file = File::open(&pool_path).unwrap();
            let reader = BufReader::new(file);
            let pool_val: serde_json::Value = serde_json::from_reader(reader).unwrap();
            println!("{:?}", pool_val);
        } else {
            let x = Pool {
                name: name.to_owned(),
                workdir: workdir.to_owned(),
                apps: vec![],
            };
            let x_str = serde_json::to_string(&x).unwrap();
            tube::fs::write_file(&pool_file, &x_str.as_bytes());
        }

        Ok(Pool {
            name: "".to_owned(),
            workdir: "".to_owned(),
            apps: vec![],
        })
    }
}