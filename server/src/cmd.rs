//! copyright © shaipe 2021 - present
//! 命令处理类
//! create by shaipe 20210120

use actix_web::{web, Error as ActixError, HttpRequest, HttpResponse};
use micro_app::{Docker, Service};
use serde::{Deserialize, Serialize};
use std::path::Path;

/// 命令处理
pub async fn handler(
    _req: HttpRequest,
    mut payload: web::Payload,
    workdir: web::Data<String>,
) -> Result<HttpResponse, ActixError> {
    use bytes::BytesMut;
    use futures::StreamExt;
    // payload is a stream of Bytes objects
    let mut body = BytesMut::new();
    while let Some(chunk) = payload.next().await {
        let chunk = chunk?;
        body.extend_from_slice(&chunk);
    }

    // 获取post的字符串
    if let Ok(s) = std::str::from_utf8(&body) {
        let mut res: Vec<String> = Vec::new();
        println!("{}", s);
        // if let Ok(v) =
        match serde_json::from_str(s) {
            Ok(v) => {
                let val: Cmd = v;

                // // println!("{:?}", val);
                let env_dir = if val.workdir.len() < 1 {
                    &val.workdir
                } else {
                    &workdir
                };

                let mut cmd = val.clone();
                // 如果传入的为空,则使用服务器配置的服务主目录
                cmd.workdir = env_dir.to_owned().clone();

                let symbol = &cmd.symbol;
                let name = &cmd.name;
                // 开始处理命令行中的变量
                cmd.start = cmd
                    .start
                    .iter()
                    .map(|x| {
                        x.replace("$symbol", symbol)
                            .replace("$name", name)
                            .replace("$workdir", &env_dir)
                    })
                    .collect();
                // 完成后执行命令变量处理
                cmd.end = cmd
                    .end
                    .iter()
                    .map(|x| {
                        x.replace("$symbol", symbol)
                            .replace("$name", name)
                            .replace("$workdir", &env_dir)
                    })
                    .collect();

                // 1. 执行开始命令
                if let Ok(s) = exec_cmd(cmd.start.clone(), env_dir) {
                    res.extend(s);
                }

                // 2. 对action进行处理
                if cmd.action.len() > 0 {
                    let file_path = Path::new(&cmd.file_path);
                    if file_path.exists() {
                        match cmd.clone().execute() {
                            Ok(s) => res.extend(s),
                            Err(err) => res.push(format!("error: {}", err)),
                        }
                    }
                }
            }
            Err(err) => res.push(format!("error: {}", err)),
        }
        // // 3. 指行命令来重新启动服务
        // if let Ok(s) = exec_cmd(&val["end"], env_dir) {
        //     res.extend(s);
        // }
        use tube_web::response;
        // 返回执行结果
        return response::get_success(&tube_value::value!(res));
    }

    Ok(HttpResponse::Ok()
        .content_type("application/json")
        .body(r#"{"error": "not support method"}"#))
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Cmd {
    action: String,
    workdir: String,
    symbol: String,
    name: String,
    #[serde(rename = "appType")]
    app_type: String,
    #[serde(rename = "filePath")]
    file_path: String,
    service: Option<Service>,
    docker: Option<Docker>,
    start: Vec<String>,
    end: Vec<String>,
}

impl Cmd {
    /// 远程命令执行
    pub fn execute(&self) -> tube_error::Result<Vec<String>> {
        if self.action == "install" {
            return self.install();
        } else {
            return self.update();
        }
    }

    /// 应用安装
    fn install(&self) -> tube_error::Result<Vec<String>> {
        let mut res: Vec<String> = Vec::new();

        // 把压缩文件解压到指定的文件夹，可直接调用一个服务器上的脚本来处理
        let fp = Path::new(&self.file_path);
        if fp.exists() {
            // 判断应用类型
            if self.app_type == "service" || self.app_type == "Service" {
                if let Some(srv) = self.service.clone() {
                    // 1. 创建目录
                    let app_path = Path::new(&srv.workdir);
                    if !app_path.exists() {
                        let _ = std::fs::create_dir_all(&app_path);
                    }

                    // 2. 解压文件
                    match tube::unzip(&self.file_path, &srv.workdir) {
                        Ok(_) => res.push("unzip successfully".to_owned()),
                        Err(err) => res.push(format!("error: {}", err)),
                    };
                    
                    // 3. 配置安装服务
                    match srv.install() {
                        Ok(v) => res.extend(v),
                        Err(err) => res.push(format!("error: {}", err)),
                    }
                }
            }
            // Docker
            else if self.app_type == "docker" || self.app_type == "Docker" {
            }
            // Files
            else {
            }
        }

        Ok(res)
    }

    /// 应用更新
    fn update(&self) -> tube_error::Result<Vec<String>> {
        let mut res: Vec<String> = Vec::new();
        let fp = Path::new(&self.file_path);
        if !fp.exists() {
            return Err(error!("file not found"));
        }
        // 判断应用类型
        if self.app_type == "service" || self.app_type == "Service" {
            if let Some(srv) = self.service.clone() {
                // 1. 停止现有服务
                match Service::stop(&srv.name) {
                    Ok(_v) => res.push("stop service successfully".to_owned()),
                    Err(err) => res.push(format!("error: {}", err)),
                }

                // 2. 备份原程序
                match srv.backup() {
                    Ok(_v) => res.push("backup service successfully".to_owned()),
                    Err(err) => res.push(format!("error: {}", err)),
                }

                // 3. 文件覆盖
                match tube::unzip(&self.file_path, &self.workdir) {
                    Ok(_) => res.push("unzip successfully".to_owned()),
                    Err(err) => res.push(format!("error: {}", err)),
                };

                // 4. 启动服务
                match Service::start(&srv.name) {
                    Ok(_v) => res.push("start service successfully".to_owned()),
                    Err(err) => res.push(format!("error: {}", err)),
                }
            }
        }
        // Docker
        else if self.app_type == "docker" || self.app_type == "Docker" {
        }
        // Files
        else {
        }

        Ok(res)
    }
}

/// 根据value执行命令
fn exec_cmd(cmds: Vec<String>, env_dir: &str) -> tube_error::Result<Vec<String>> {
    let mut res: Vec<String> = Vec::new();
    for cmd in cmds {
        match run_cmd(&cmd, env_dir, true) {
            Ok(t) => {
                // let tc: Vec<String> = t.split("\n").map(|s| s.to_owned()).collect();
                res.extend(t);
            }
            Err(err) => {
                res.push(format!("error: {}", err));
            }
        }
    }
    Ok(res)
}

/// 运行命令
pub(crate) fn run_cmd(
    cmd: &str,
    env_dir: &str,
    enable_capture: bool,
) -> tube_error::Result<Vec<String>> {
    use tube_cmd::Command;
    // let cmd = Command::with_args("bash", &["-c", "ls ; sleep 2; ls"]).set_dir(env_dir).add_args(&[cmd]);
    // 对操作系统进行判断
    let cmd_name = if cfg!(target_os = "Windows") {
        "ps"
    } else {
        "bash"
    };

    let res = if enable_capture {
        Command::with_args(cmd_name, &["-c", cmd])
            .set_dir(env_dir.clone())
            .enable_capture()
            .run()
    } else {
        Command::with_args(cmd_name, &["-c", cmd])
            .set_dir(env_dir.clone())
            .run()
    };

    let res = match res {
        Ok(s) => format!("{}", s.stdout_string_lossy()),
        Err(e) => {
            // println!("{:?}", e);
            format!("{:?}", e.to_string())
        }
    };
    let x = res.lines().map(|x| x.to_owned()).collect::<Vec<String>>();
    Ok(x)
}
