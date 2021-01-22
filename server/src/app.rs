//! copyright © shaipe 2021 - present
//! 微服务应用管理
//! create by shaipe 20210123

use actix_web::{web, Error as ActixError, HttpRequest, HttpResponse, Result as ActixResult};
use serde::{Deserialize, Serialize};
use tube_error::Result;

use crate::config::get_config;

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

/// 应用

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct App {
    // 名称
    pub name: String,
    // 标题
    pub description: String,
    // 应用工作目录
    pub workdir: String,
    // 端口
    pub port: u16,
    // 应用状态
    pub status: u16,
}

impl App {
    /// 新建应用对象
    pub fn new(name: String) -> App {
        App {
            name,
            description: "".to_owned(),
            workdir: "./".to_owned(),
            port: 7000,
            status: 0,
        }
    }
}

async fn distribute(action: &str) {
    match action {
        "install" => {}
        _ => {}
    }
}

/// 微服务应用池管理。
async fn handler(
    _req: HttpRequest,
    mut payload: web::Payload,
    pool: web::Data<Pool>,
) -> ActixResult<HttpResponse, ActixError> {
    use bytes::BytesMut;
    use futures::StreamExt;
    use tube_web::response;

    println!("{:?}", pool);

    // payload is a stream of Bytes objects
    let mut body = BytesMut::new();
    while let Some(chunk) = payload.next().await {
        let chunk = chunk?;
        body.extend_from_slice(&chunk);
    }

    if let Ok(s) = std::str::from_utf8(&body) {
        println!("req string::: {}", s);
        let val: serde_json::Value = serde_json::from_str(s).unwrap();
        println!("{:?}", val);
        // 返回执行结果
        return response::get_success(&tube_value::value!("res"));
    }

    response::get_error(error!("not found execute method"))
}

/// api接口配置
pub fn app_config(scf: &mut web::ServiceConfig) {
    if let Some(cnf) = super::config::get_config() {
        // println!("{:?}", cnf);
        let pool = Pool::load(&cnf.workdir, "orion").unwrap();
        scf.service(
            // api 目录代理
            web::scope("/app")
                .data(pool.clone())
                .route("", web::post().to(handler)),
        );
    }
}
