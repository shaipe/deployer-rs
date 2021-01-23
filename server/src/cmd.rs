//! copyright © shaipe 2021 - present
//! 命令处理类
//! create by shaipe 20210120

use actix_web::{web, Error as ActixError, HttpRequest, HttpResponse};

/// 命令处理
pub async fn handler(
    _req: HttpRequest,
    mut payload: web::Payload,
) -> Result<HttpResponse, ActixError> {
    use bytes::BytesMut;
    use futures::StreamExt;
    // payload is a stream of Bytes objects
    let mut body = BytesMut::new();
    while let Some(chunk) = payload.next().await {
        let chunk = chunk?;
        body.extend_from_slice(&chunk);
    }

    if let Ok(s) = std::str::from_utf8(&body) {
        let val: serde_json::Value = serde_json::from_str(s).unwrap();

        // println!("{:?}", val);

        let env_dir = match val["workdir"].as_str() {
            Some(x) => x,
            None => "./",
        };

        let mut res = Vec::new();

        // 1. 对文件进行解压前的命令处理
        if let Some(cmds) = val["startCommand"].as_array() {
            for cmd in cmds {
                if let Some(c) = cmd.as_str() {
                    match run_cmd(c, env_dir, true) {
                        Ok(t) => {
                            // let tc: Vec<String> = t.split("\n").map(|s| s.to_owned()).collect();
                            res.extend(t);
                        }
                        Err(err) => {
                            res.push(format!("error: {}", err));
                        }
                    }
                }
            }
        }
        // println!("env:: {}, {:?}", env_dir, val["data"]["relativePath"]);
        // 2. 把压缩文件解压到指定的文件夹，可直接调用一个服务器上的脚本来处理
        // if let Some(p) = val["data"]["relativePath"].as_str() {
        //     if p.len() > 1 {
        //         // println!("server unzip {:?}, {:?}",p, env_dir);
        //         match tube::unzip(p, env_dir) {
        //             Ok(_) => {}
        //             Err(err) => res.push(format!("error: {}", err)),
        //         };
        //     }
        // }

        // 3. 指行命令来重新启动服务
        if let Some(cmds) = val["endCommand"].as_array() {
            for cmd in cmds {
                if let Some(c) = cmd.as_str() {
                    match run_cmd(c, env_dir, true) {
                        Ok(t) => {
                            // let tc: Vec<String> = t.split("\n").map(|s| s.to_owned()).collect();
                            res.extend(t);
                        }
                        Err(err) => {
                            res.push(format!("error: {}", err));
                        }
                    }
                }
            }
        }

        use tube_web::response;
        // 返回执行结果
        return response::get_success(&tube_value::value!(res));
    }

    // let hello = match Command::with_args("bash", &["-c", "ls ; sleep 2; ls"])
    //     .enable_capture()
    //     .run()
    // {
    //     Ok(s) => format!("{}", s.stdout_string_lossy()),
    //     Err(e) => {
    //         // println!("{:?}", e);
    //         format!("{:?}", e.to_string())
    //     }
    // };
    // println!("{:?}", hello);

    Ok(HttpResponse::Ok()
        .content_type("application/json")
        .body(r#"{"error": "not support method"}"#))
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
