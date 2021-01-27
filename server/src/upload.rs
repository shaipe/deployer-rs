//! copyright © shaipe 2021 - present
//! 文件上传处理类
//! create by shaipe 20210120

use actix_multipart::Multipart;
use actix_web::{Error as ActixError, HttpRequest, HttpResponse};
use oss::save_file;
use tube_web::response;

/// 文件上传处理
pub async fn handler(
    req: HttpRequest,
    payload: Multipart,
    // srv: web::Data<Addr<ws::WsServer>>,
) -> Result<HttpResponse, ActixError> {
    // println!("{:?}", req);
    use tube_value::{ToValue, Value};
    // let root_dir = format!("{}/userfiles", std::env::current_dir().unwrap().display());
    // println!("{}", root_dir);
    match save_file(req, payload, "userfiles", false).await {
        Ok((res, _forms)) => {
            if res.len() < 2 {
                response::get_success(&res[0].to_value())
            } else {
                response::get_success(&Value::Null)
            }
        }
        Err(e) => response::get_error(e),
    }
}
