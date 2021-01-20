//! copyright © shaipe 2021 - present
//! 服务端应用
//! create by shaipe 20210102

#[macro_use]
extern crate tube_error;
extern crate oss;
mod config;

// 在主文件中必须要引入Error类型,来定义整个包的基础错误类型
use actix_multipart::Multipart;
use actix_web::{middleware, web, App, Error as ActixError, HttpRequest, HttpResponse, HttpServer};
use config::Config;
use oss::save_file;
use tube_cmd::Command;
use tube_error::Error;
use tube_web::response;

/// 首页
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

/// 文件上传处理
pub async fn upload_handler(
    req: HttpRequest,
    payload: Multipart,
    // srv: web::Data<Addr<ws::WsServer>>,
) -> Result<HttpResponse, ActixError> {
    println!("{:?}", req);
    use tube_value::{ToValue, Value};
    match save_file(req, payload, "userfiles").await {
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

/// 命令处理
async fn cmd_handler(
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

        // 1. 把压缩文件解压到指定的文件夹，可直接调用一个服务器上的脚本来处理
        // 2. 指行命令来重新启动服务
        if let Some(cmds) = val["commands"].as_array() {
            let env_dir = match val["workdir"].as_str() {
                Some(x) => x,
                None => "./",
            };
            let mut res = Vec::new();
            for cmd in cmds {
                if let Some(c) = cmd.as_str() {
                    match run_cmd(c, env_dir, true) {
                        Ok(t) => {
                            let tc:Vec<String> = t.split("\n").map(|s| s.to_owned()).collect();
                            res.extend(tc);
                        }
                        Err(err) => {
                            res.push(format!("error: {}", err));
                        }
                    }
                }
            }
            return response::get_success(&tube_value::value!(res));
        }
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
        App::new()
            .wrap(middleware::Logger::default())
            .service(web::resource("/cmd").route(web::post().to(cmd_handler)))
            .service(
                web::resource("/")
                    .route(web::get().to(index))
                    .route(web::post().to(upload_handler)),
            )
    })
    .bind(ip)?
    .run()
    .await
}

/// 运行命令
pub fn run_cmd(cmd: &str, env_dir: &str, enable_capture: bool) -> tube_error::Result<String> {
    // let cmd = Command::with_args("bash", &["-c", "ls ; sleep 2; ls"]).set_dir(env_dir).add_args(&[cmd]);

    let res = if enable_capture {
        Command::with_args("bash", &["-c", cmd])
            .set_dir(env_dir)
            .enable_capture()
            .run()
    } else {
        Command::with_args("bash", &["-c", cmd])
            .set_dir(env_dir)
            .run()
    };

    let hello = match res {
        Ok(s) => format!("{}", s.stdout_string_lossy()),
        Err(e) => {
            // println!("{:?}", e);
            format!("{:?}", e.to_string())
        }
    };
    Ok(hello)
}
