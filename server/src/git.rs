//! copyright © ecdata.cn 2021 - present
//! GIT Hooks 及GIT相关工具处理
//! create by shaipe 20210102

use actix_web::{web, Error as ActixError, HttpRequest, HttpResponse};
// use tube::cmd::Command;

/// 命令处理
/// WebHook 简介 Gitee WebHook 功能是帮助用户 push 代码后，自动回调一个您设定的 http 地址。
pub async fn handler(
    _req: HttpRequest,
    mut payload: web::Payload,
    workdir: web::Data<String>,
) -> Result<HttpResponse, ActixError> {
    use bytes::BytesMut;
    use futures::StreamExt;
    use tube_web::response;

    // payload is a stream of Bytes objects
    let mut body = BytesMut::new();
    while let Some(chunk) = payload.next().await {
        let chunk = chunk?;
        body.extend_from_slice(&chunk);
    }

    if let Ok(s) = std::str::from_utf8(&body) {
        println!("req string::: {}", s);
        let val: serde_json::Value = serde_json::from_str(s).unwrap();
        let mut res: Vec<String> = Vec::new();
        if let Some(action) = val["action"].as_str() {
            let env_dir = match val["workdir"].as_str() {
                Some(dir) => dir,
                None => &workdir,
            };
            let git = tube::git::Git::new(env_dir);
            match action {
                "clone" => {
                    if let Some(url) = val["url"].as_str() {
                        match tube::git::Git::clone(env_dir, url, false) {
                            Ok(r) => res.extend(r),
                            Err(e) => res.push(format!("error:{}", e)),
                        }
                    }
                }
                "pull" => match git.pull() {
                    Ok(r) => res.extend(r),
                    Err(e) => res.push(format!("error:{}", e)),
                },
                "push" => match git.push() {
                    Ok(r) => res.extend(r),
                    Err(e) => res.push(format!("error:{}", e)),
                },
                "commit" => {
                    if let Some(msg) = val["message"].as_str() {
                        match git.commit(msg) {
                            Ok(r) => res.extend(r),
                            Err(e) => res.push(format!("error:{}", e)),
                        }
                    }
                }
                "branch" => {
                    let is_all = if let Some(all) = val["all"].as_str() {
                        all == "true" || all == "True"
                    } else {
                        false
                    };
                    match git.branch(is_all) {
                        Ok(r) => res.extend(r),
                        Err(e) => res.push(format!("error:{}", e)),
                    }
                }
                _ => {}
            }
        }

        println!("{:?}", val);
        // 返回执行结果
        return response::get_success(&tube_value::value!(res));
    }

    response::get_error(error!("not found execute method"))
}


/// 命令处理
/// WebHook 简介 Gitee WebHook 功能是帮助用户 push 代码后，自动回调一个您设定的 http 地址。
pub async fn hook(
    _req: HttpRequest,
    mut payload: web::Payload,
) -> Result<HttpResponse, ActixError> {
    use bytes::BytesMut;
    use futures::StreamExt;
    use tube_web::response;

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