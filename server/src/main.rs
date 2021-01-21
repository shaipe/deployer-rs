//! copyright © shaipe 2021 - present
//! 服务端应用
//! create by shaipe 20210102

#[macro_use]
extern crate tube_error;
extern crate oss;
mod cmd;
mod config;
mod upload;

use actix_web::{middleware, web, App, HttpServer};
use config::Config;
// 在主文件中必须要引入Error类型,来定义整个包的基础错误类型
use tube_error::Error;

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
            .service(web::resource("/cmd").route(web::post().to(cmd::handler)))
            .service(
                web::resource("/upload").route(web::post().to(upload::handler)), // .route(web::post().to(upload::handler)),
            )
    })
    .bind(ip)?
    .run()
    .await
}

// /// 首页
// async fn index() -> HttpResponse {
//     // Begin readme example
//     // This will return an error if the command did not exit successfully
//     // (controlled with the `check` field).
//     let hello = match Command::with_args("bash", &["-c", "ls ; sleep 2; ls"])
//         .enable_capture()
//         .run()
//     {
//         Ok(s) => format!("{}", s.stdout_string_lossy()),
//         Err(e) => {
//             // println!("{:?}", e);
//             format!("{:?}", e.to_string())
//         }
//     };

//     // let hello = output.stdout_string_lossy();

//     // println!("{}", hello);

//     let html = format!(
//         r#"<html>
//     <head><title>Upload Test</title></head>
//     <body>
//         <div>{:?}</div>
//         <form target="/" method="post" enctype="multipart/form-data">
//             <input type="file" multiple name="file"/>
//             <button type="submit">Submit</button>
//         </form>
//     </body>
// </html>"#,
//         hello
//     );

//     HttpResponse::Ok().body(html)
// }
