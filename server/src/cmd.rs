//! copyright © ecdata.cn 2021 - present
//! 命令处理类
//! create by shaipe 20210120

use actix_web::{web, Error as ActixError, HttpRequest, HttpResponse};
use micro_app::{Docker, Service};
use serde::{Deserialize, Serialize};
use std::path::Path;
use std::time::Duration;
use std::thread;

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
                let wwk = &workdir.clone();
                // // println!("{:?}", val);
                let env_dir = if let Some(wk) = &val.workdir {
                    if wk.len() > 0 {
                        wk
                    } else {
                        &wwk
                    }
                } else {
                    &wwk
                };

                let mut cmd = val.clone();
                // 如果传入的为空,则使用服务器配置的服务主目录
                cmd.workdir = Some(env_dir.to_owned().clone());

                let symbol = match &cmd.symbol {
                    Some(s) => &s,
                    None => "",
                };

                let name = match &cmd.name {
                    Some(s) => s.clone(),
                    None => "".to_owned(),
                };

                if let Some(sc) = cmd.start {
                    // 开始处理命令行中的变量
                    cmd.start = Some(
                        sc.iter()
                            .map(|x| {
                                x.replace("$symbol", &symbol)
                                    .replace("$name", &name)
                                    .replace("$workdir", &env_dir)
                            })
                            .collect(),
                    );
                }

                // 完成后执行命令变量处理
                if let Some(ec) = cmd.end.clone() {
                    // 开始处理命令行中的变量
                    cmd.start = Some(
                        ec.iter()
                            .map(|x| {
                                x.replace("$symbol", symbol)
                                    .replace("$name", &name)
                                    .replace("$workdir", &env_dir)
                            })
                            .collect(),
                    );
                }

                // 1. 执行开始命令
                if let Some(start_cmd) = cmd.start.clone() {
                    if let Ok(s) = exec_cmd(start_cmd.clone(), env_dir) {
                        res.extend(s);
                    }
                }

                // 2. 对action进行处理
                if let Some(act) = cmd.clone().action {
                    if act.len() > 0 {
                        match cmd.execute() {
                            Ok(s) => res.extend(s),
                            Err(err) => res.push(format!("error: {}", err)),
                        }
                    }
                } else if let Some(cmds) = cmd.command {
                    let wk_dir = match cmd.workdir {
                        Some(dir) => dir,
                        None => "./".to_owned(),
                    };
                    match exec_cmd(cmds, &wk_dir) {
                        Ok(s) => res.extend(s),
                        Err(err) => res.push(format!("error: {}", err)),
                    }
                }

                // 3. 执行结束命令
                if let Some(end_cmd) = cmd.end.clone() {
                    if let Ok(s) = exec_cmd(end_cmd.clone(), env_dir) {
                        res.extend(s);
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
    action: Option<String>,
    workdir: Option<String>,
    symbol: Option<String>,
    name: Option<String>,
    #[serde(rename = "appType")]
    app_type: Option<String>,
    #[serde(rename = "filePath")]
    file_path: Option<String>,
    service: Option<Service>,
    docker: Option<Docker>,
    start: Option<Vec<String>>,
    end: Option<Vec<String>>,
    command: Option<Vec<String>>,
}

impl Cmd {
    /// 远程命令执行
    pub fn execute(&self) -> tube_error::Result<Vec<String>> {
        if let Some(act) = self.clone().action {
            if act == "install" {
                return self.install();
            } else {
                return self.update();
            }
        }
        Ok(vec![])
    }

    /// 应用安装
    fn install(&self) -> tube_error::Result<Vec<String>> {
        let mut res: Vec<String> = Vec::new();

        // 把压缩文件解压到指定的文件夹，可直接调用一个服务器上的脚本来处理
        if let Some(f_path) = self.clone().file_path {
            let fp = Path::new(&f_path);
            if fp.exists() {
                if let Some(app_type) = self.clone().app_type {
                    // 判断应用类型
                    if app_type == "service" || app_type == "Service" {
                        if let Some(srv) = self.service.clone() {
                            // 1. 创建目录
                            let app_path = Path::new(&srv.workdir);
                            if !app_path.exists() {
                                let _ = std::fs::create_dir_all(&app_path);
                            }

                            // 2. 解压文件
                            match tube::unzip(&f_path, &srv.workdir) {
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
                    else if app_type == "docker" || app_type == "Docker" {
                    }
                    // Files
                    else {
                        match self.files_deal() {
                            Ok(v) => res.extend(v),
                            Err(err) => res.push(format!("error: {}", err)),
                        }
                    }
                }
            }
        }
        Ok(res)
    }

    /// 应用更新
    fn update(&self) -> tube_error::Result<Vec<String>> {
        let mut res: Vec<String> = Vec::new();

        if let Some(f_path) = self.clone().file_path {
            let fp = Path::new(&f_path);
            if !fp.exists() {
                return Err(error!("file not found"));
            }
            if let Some(app_type) = self.clone().app_type {
                // 判断应用类型
                if app_type == "service" || app_type == "Service" {
                    if let Some(srv) = self.service.clone() {
                        // 1. 停止现有服务
                        match Service::stop(&srv.name) {
                            Ok(_v) => res.push("stop service successfully".to_owned()),
                            Err(err) => res.push(format!("error: {}", err)),
                        }

                        // 延迟1秒现进行程序更新
                        thread::sleep(Duration::from_millis(1000));

                        // 2. 备份原程序
                        match srv.backup() {
                            Ok(_v) => res.push("backup service successfully".to_owned()),
                            Err(err) => res.push(format!("error: {}", err)),
                        }

                        // 3. 文件覆盖
                        match srv.unzip(&f_path) {
                            Ok(s) => res.extend(s),
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
                else if app_type == "docker" || app_type == "Docker" {
                }
                // Files
                else {
                    match self.files_deal() {
                        Ok(v) => res.extend(v),
                        Err(err) => res.push(format!("error: {}", err)),
                    }
                }
            }
        }

        Ok(res)
    }

    /// 文件的相关处理
    fn files_deal(&self) -> tube_error::Result<Vec<String>> {
        let mut res = Vec::new();
        if let Some(f_path) = self.clone().file_path {
            let file_path = Path::new(&f_path);
            if file_path.exists() {
                // 1. 创建目录
                if let Some(wk) = self.clone().workdir {
                    let dir_path = Path::new(&wk);
                    if !dir_path.exists() {
                        let _ = std::fs::create_dir_all(&dir_path);
                        res.push(format!("crate dir {} successfully", wk));
                    }

                    // 获取文件名
                    let file_name = match file_path.file_name() {
                        Some(f) => f.to_str().unwrap(),
                        None => "",
                    };

                    // 复制目标路径
                    let to_file = format!("{}/{}", wk, file_name);
                    // 2. 文件复制
                    match std::fs::copy(file_path, &to_file) {
                        Ok(_) => res.push(format!("copy {} successfully", f_path)),
                        Err(err) => res.push(format!("error: {}", err)),
                    }
                }
            }
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
    use tube::cmd::Command;
    // let cmd = Command::with_args("bash", &["-c", "ls ; sleep 2; ls"]).set_dir(env_dir).add_args(&[cmd]);
    // 对操作系统进行判断
    let cmd_name = if cfg!(target_os = "Windows") {
        "ps"
    } else {
        "bash"
    };

    let res = if enable_capture {
        Command::with_args(cmd_name, &["-c", cmd])
            .set_dir(env_dir)
            .enable_capture()
            .run()
    } else {
        Command::with_args(cmd_name, &["-c", cmd])
            .set_dir(env_dir)
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
