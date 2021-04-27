//! copyright © ecdata.cn 2021 - present
//! 日志查看工具
//! create by shaipe 20210123

use actix_web::{web, Error as ActixError, HttpRequest, HttpResponse};
// use tube::cmd::Command;

struct GitArgument {
    workdir: String,
    action: String,
    arguments: Vec<String>,
}

/// 命令处理
/// WebHook 简介 Gitee WebHook 功能是帮助用户 push 代码后，自动回调一个您设定的 http 地址。
pub async fn list(
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


/// api接口配置
pub fn log_config(scf: &mut web::ServiceConfig) {
    if let Some(cnf) = super::config::get_config() {
        // println!("{:?}", cnf);
        // let pool = Pool::load(&cnf.workdir, "orion").unwrap();
        scf.service(
            // api 目录代理
            web::scope("/log")
                .data(cnf.workdir.clone())
                .route("", web::get().to(handler)),
        );
    }
}
