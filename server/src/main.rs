//! copyright © shaipe 2021 - present
//! 服务端应用
//! create by shaipe 20210102

#[macro_use]
extern crate tube_error;

// 在主文件中必须要引入Error类型,来定义整个包的基础错误类型
use tube_error::Error;

use actix_web::{middleware, web, App, HttpResponse, HttpServer};

mod config;
use config::Config;

mod upload;
use upload::upload_handler;

fn index() -> HttpResponse {
    let html = r#"<html>
        <head><title>Upload Test</title></head>
        <body>
            <form target="/" method="post" enctype="multipart/form-data">
                <input type="file" multiple name="file"/>
                <button type="submit">Submit</button>
            </form>
        </body>
    </html>"#;

    HttpResponse::Ok().body(html)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_server=info,actix_web=info");
    std::fs::create_dir_all("./tmp").unwrap();

    let conf = match Config::new("conf/server.yml") {
        Ok(conf) => conf,
        Err(e) => {
            println!("{:?}", e);
            Config::default()
        }
    };

    println!("config {:?}", conf);

    let ip = format!("{}:{}", conf.server.ip, conf.server.port);
    println!("{:?}", ip);

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
