//! copyright © ecdata.cn 2021 - present
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
            // println!("{:?}", res);
            if res.len() > 0 {
                response::get_success(&res[0].to_value())
            } else {
                response::get_success(&Value::Null)
            }
        }
        Err(e) => response::get_error(e),
    }
}

/// 首页
pub async fn get() -> HttpResponse {
    // Begin readme example
    // This will return an error if the command did not exit successfully
    // (controlled with the `check` field).
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

    // let hello = output.stdout_string_lossy();

    // println!("{}", hello);

    let html = format!(
        r#"<html>
    <head><title>Upload Test</title><meta charset='utf-8'></head>
    <body>
        <form target="/" method="post" enctype="multipart/form-data">
            <input type="file" multiple name="file"/>
            <button type="submit">Submit</button>
        </form>
    </body>
</html>"#
    );

    HttpResponse::Ok().body(html)
}
