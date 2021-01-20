//! copyright © shaipe 2021 - present
//! 服务端应用
//! create by shaipe 20210102

#[macro_use]
extern crate tube_error;
extern crate oss;

// 在主文件中必须要引入Error类型,来定义整个包的基础错误类型
use actix_multipart::Multipart;
use actix_web::{middleware, web, App, Error as ActixError, HttpRequest, HttpResponse, HttpServer};
use tube_error::Error;

mod config;
use config::Config;
use tube_cmd::Command;

use oss::save_file;
mod upload;

async fn index() -> HttpResponse {
    // Begin readme example
    // This will return an error if the command did not exit successfully
    // (controlled with the `check` field).
    let hello = match Command::with_args("bash", &["-c", "ls ; sleep 2; ls"])
        .enable_capture()
        .run()
    {
        Ok(s) => format!("{}", s.stdout_string_lossy()),
        Err(e) => {
            // println!("{:?}", e);
            format!("{:?}", e.to_string())
        }
    };

    // let hello = output.stdout_string_lossy();

    // println!("{}", hello);

    let html = format!(
        r#"<html>
    <head><title>Upload Test</title></head>
    <body>
        <div>{:?}</div>
        <form target="/" method="post" enctype="multipart/form-data">
            <input type="file" multiple name="file"/>
            <button type="submit">Submit</button>
        </form>
    </body>
</html>"#,
        hello
    );

    HttpResponse::Ok().body(html)
}

use tube_web::response;

pub async fn upload_handler(
    req: HttpRequest,
    payload: Multipart,
    // srv: web::Data<Addr<ws::WsServer>>,
) -> Result<HttpResponse, ActixError> {
    println!("{:?}", req);
    use tube_value::{Value,ToValue};
    match save_file(req, payload, "userfiles").await {
        Ok((res, _forms)) => {
            if res.len() <2 {
                response::get_success(&res[0].to_value())
            }
            else{
                response::get_success(&Value::Null)
            }
        },
        Err(e) => response::get_error(e)
    }
   
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_server=info,actix_web=info");
    // std::fs::create_dir_all("./tmp").unwrap();

    let conf = match Config::new("conf/server.yml") {
        Ok(conf) => conf,
        Err(e) => {
            println!("{:?}", e);
            Config::default()
        }
    };

    let ip = format!("{}:{}", conf.server.ip, conf.server.port);

    HttpServer::new(|| {
        App::new().wrap(middleware::Logger::default()).service(
            web::resource("/")
                .route(web::get().to(index))
                .route(web::post().to(upload_handler)),
        )
    })
    .bind(ip)?
    .run()
    .await
}
