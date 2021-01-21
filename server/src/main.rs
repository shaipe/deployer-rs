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
use clap::{crate_authors, crate_description, crate_version, App as ClapApp, Arg};
use tube_error::Error;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_server=info,actix_web=info");

    // 获取命令行参数
    let matches = ClapApp::new("dserver")
        .version(crate_version!())
        .author(crate_authors!())
        .about(crate_description!())
        // .setting(AppSettings::SubcommandRequiredElseHelp)
        .arg(
            Arg::with_name("config")
                .short("c")
                .long("config")
                .value_name("FILE")
                // .help("set name of program")
                .takes_value(true),
        )
        .subcommand(
            ClapApp::new("install")
                .about("service install")
                .version("0.1.0")
                .author("Shaipe<shaipe@sina.com>")
                .arg(Arg::with_name("debug").short("d")),
        )
        .subcommand(
            ClapApp::new("uninstall")
                .about("service uninstall")
                .version("0.1.0")
                .author("Shaipe<shaipe@sina.com>")
                .arg(Arg::with_name("debug").short("d")),
        )
        .get_matches();

    let (sub_cmd, _) = matches.subcommand();

    // 对子命令进行处理
    if sub_cmd.len() > 0 {
        if sub_cmd == "install" {
            println!("install");
        }
        else if sub_cmd == "uninstall" {
            println!("uninstall");
        }
        println!("execute completed");
        return Ok(());
    }

    // 加载配置文件
    let conf_path = matches.value_of("config").unwrap_or("conf/server.yml");

    // 加载配置信息
    let conf = match Config::new(conf_path) {
        Ok(conf) => conf,
        Err(e) => {
            println!("file: {}, {:?}", conf_path, e);
            Config::default()
        }
    };

    // 设置服务器运行ip和端口信息
    let ip = format!("{}:{}", conf.server.ip, conf.server.port);

    // 启动一个web服务
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
