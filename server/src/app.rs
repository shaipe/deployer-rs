//! copyright © shaipe 2021 - present
//! 微服务应用管理
//! create by shaipe 20210123

use actix_web::{web, Error as ActixError, HttpRequest, HttpResponse, Result as ActixResult};
use serde::{Deserialize, Serialize};
use tube_error::Result;
use micro_app::Pool;

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
