//! copyright © shaipe 2021 - present
//! 命令处理类
//! create by shaipe 20210120

use actix_web::{web, Error as ActixError, HttpRequest, HttpResponse};
use micro_app::App;
use serde::{Deserialize, Serialize};
use serde_json::Value;
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
        if let Ok(v) = serde_json::from_str(s) {
            let val: Cmd = v;
            println!("{:?}", val);

            // // println!("{:?}", val);
            let env_dir = if val.workdir.len() < 1 {
                &val.workdir
            } else {
                &workdir
            };

            // 1. 执行开始命令
            if let Ok(s) = exec_cmd(val.start, env_dir) {
                res.extend(s);
            }

            // 2. 对action进行处理
            if val.action.len() > 0 {
                let file_path = Path::new(&val.file_path);
                if file_path.exists() {
                    match val.action.as_str() {
                        "install" => match install(env_dir, &val.file_path, val.app) {
                            Ok(s) => res.extend(s),
                            Err(err) => res.push(format!("error: {}", err)),
                        },
                        "update" => match install(env_dir, &val.file_path, val.app) {
                            Ok(s) => res.extend(s),
                            Err(err) => res.push(format!("error: {}", err)),
                        },
                        _ => {}
                    }
                }
            }
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
    workdir: String,
    app: App,
    action: String,
    #[serde(rename = "filePath")]
    file_path: String,
    start: Vec<String>,
    end: Vec<String>,
}

/// 应用安装
fn install(workdir: &str, file_path: &str, app: App) -> tube_error::Result<Vec<String>> {
    let mut res: Vec<String> = Vec::new();
    // 1. 创建目录
    let app_dir = format!("{}/{}/{}", workdir, app.symbol, app.name);
    let app_path = Path::new(&app_dir);
    if !app_path.exists() {
        let _ = std::fs::create_dir_all(&app_path);
    }

    // 2. 把压缩文件解压到指定的文件夹，可直接调用一个服务器上的脚本来处理
    let fp = Path::new(file_path);
    if fp.exists() {
        match tube::unzip(file_path, &app_dir) {
            Ok(_) => {}
            Err(err) => res.push(format!("error: {}", err)),
        };
    }

    // 3. 配置安装服务
    if app.is_service && app.exec_start.len() > 0 {
        let exec_path = Path::new(&app.exec_start);
        // 是否安装服务
        if exec_path.exists() {
            match app.install_service() {
                Ok(b) => {
                    res.push(format!("install service status {}", b));
                }
                Err(err) => res.push(format!("error: {}", err)),
            }
        }
    }

    Ok(vec![])
}

/// 应用更新
fn update() -> tube_error::Result<Vec<String>> {
    Ok(vec![])
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
