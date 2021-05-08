//! copyright © ecdata.cn 2021 - present
//! 服务部署器客户端工具
//! create by shaipe 20210102
#[macro_use]
extern crate tube_error;

// 在主文件中必须要引入Error类型,来定义整个包的基础错误类型
use tube_error::Error;

mod config;
mod service;
pub(crate) use service::TaskService;

use clap::{crate_authors, crate_description, crate_version, App, Arg};
use config::Config;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 获取命令行参数
    let matches = App::new("dcli")
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
            App::new("install")
                .about("app build and remote install")
                .version("0.1.0")
                .author("Shaipe E. <shaipe@sina.com>")
                .arg(
                    Arg::with_name("name")
                        .short("n")
                        .long("name")
                        // .value_name("FILE")
                        .help("需要处理的任务名，all为全部任务，可以和e参数配合排除")
                        .takes_value(true),
                ).arg(
                    Arg::with_name("exclude")
                        .short("e")
                        .long("exclude")
                        .takes_value(true)
                        .help("输入需要排队的任务名，多个用逗号隔开。"),
                ),
        )
        .subcommand(
            App::new("update")
                .about("app build and remote update")
                .version("0.1.0")
                .author("Shaipe E. <shaipe@sina.com>")
                .arg(
                    Arg::with_name("name")
                        .short("n")
                        .long("name")
                        .takes_value(true)
                        .help("需要处理的任务名，all为全部任务，可以和e参数配合排除"),
                )
                .arg(
                    Arg::with_name("exclude")
                        .short("e")
                        .long("exclude")
                        .takes_value(true)
                        .help("输入需要排队的任务名，多个用逗号隔开。"),
                ),
        )
        .get_matches();

    // 加载配置文件
    let conf_path = matches.value_of("config").unwrap_or("conf/cli.yml");

    let (sub_cmd, sub_args) = matches.subcommand();

    let mut cf = conf_path.to_owned();

    // 只有非调试模式下才使用下面的配置
    if !cfg!(debug_assertions) {
        // 给定了相对顶层路径时不处理
        if !conf_path.starts_with("/") {
            if let Ok(p) = std::env::current_exe() {
                let workdir = format!("{}", p.parent().unwrap().display());
                cf = format!("{}/{}", &workdir, conf_path.replace("./", ""));
            }
        }
    }

    // println!("Value for config: {}", cf.clone());

    let cnf = match Config::new(&cf) {
        Ok(c) => c,
        Err(_e) => panic!("配置文件加载错误."),
    };

    // println!("c::{:?}", cnf);

    println!("start sub command {} ...", sub_cmd);

    // 对子命令进行处理
    if sub_cmd.len() > 0 {
        // 应用安装
        if sub_cmd == "install" {
            if let Some(sub_matches) = sub_args {
                if let Some(name) = sub_matches.value_of("name") {
                    // println!("name {}", name);
                    if name.to_lowercase() == "all" {
                        // 获取要排除的任务名
                        let ss = match sub_matches.value_of("exclude") {
                            Some(s) => s.split(",").map(|s| s.to_lowercase()).collect(),
                            None => vec![],
                        };
                        for tsk in cnf.tasks {
                            if ss.contains(&tsk.name.clone()) {
                                println!("exclude task {} ...", tsk.name);
                            } else {
                                println!("start {} task process", tsk.name);
                                let res = tsk.install();
                                output_msg(res);
                            }
                        }
                    } else if let Some(tsk) = cnf.get_task(name) {
                        let res = tsk.install();
                        output_msg(res);
                    }
                    else{
                        println!("not found task {}", name);
                    }
                }
            }
        }
        // 应用更新
        else if sub_cmd == "update" {
            if let Some(sub_matches) = sub_args {
                if let Some(name) = sub_matches.value_of("name") {
                    if name.to_lowercase() == "all" {
                        // 获取要排除的任务名
                        let ss = match sub_matches.value_of("exclude") {
                            Some(s) => s.split(",").map(|s| s.to_lowercase()).collect(),
                            None => vec![],
                        };
                        for tsk in cnf.tasks {
                            if ss.contains(&tsk.name.clone()) {
                                println!("exclude task {} ...", tsk.name);
                            } else {
                                println!("start {} task process", tsk.name);
                                let res = tsk.update();
                                output_msg(res);
                            }
                        }
                    } else {
                        if let Some(tsk) = cnf.get_task(name) {
                            let res = tsk.update();
                            output_msg(res);
                        }
                    }
                }
            }
        }
        // 执行子命令结束
        return Ok(());
    }

    // 1. 开始执行的命令
    // for lc in cnf.local.start_cmd {
    //     println!("cmd::{}", lc);
    //     let cmd_res = cmd::run_cmd(&lc, &cnf.local.workdir, false);
    //     println!("{:?}", cmd_res);
    // }

    // // 2. 复制并上传文件
    // // remote::upload_img();
    // let f_str = format!("{}/{}", cnf.local.workdir, cnf.local.upload_file);
    // // let f_str = "/Users/shaipe/Documents/xlsx/order.csv";
    // let f_path = Path::new(&f_str);
    // let name = f_path.file_stem().unwrap().to_str().unwrap();
    // let up_res = upload_file(&cnf.local.upload_url, name.to_owned(), f_path, None);

    // // 判断上传是否成功
    // // 3. 调用执行远端命令
    // if let Ok(res) = up_res {
    //     if res.len() > 0 {
    //         let yy = remote::call_remote(
    //             &cnf.remote.uri,
    //             serde_json::json!({
    //                 "workdir": cnf.remote.workdir,
    //                 "data": {
    //                     "relativePath": res
    //                 },
    //                 "startCommand": cnf.remote.start_cmd,
    //                 "endCommand": cnf.remote.end_cmd
    //             }),
    //         );
    //         println!("{:?}", yy);
    //     }
    // }

    // // 4. 完成后执行的本地命令
    // for lc in cnf.local.end_cmd {
    //     println!("cmd::{}", lc);
    //     let cmd_res = cmd::run_cmd(&lc, &cnf.local.workdir, false);
    //     println!("{:?}", cmd_res);
    // }

    // println!("自动更新命令完成...");

    Ok(())
}

fn output_msg(res: tube_error::Result<Vec<String>>) {
    match res {
        Ok(t) => {
            for s in t {
                println!("{}", s);
            }
        }
        Err(err) => println!("error: {}", err),
    }
}

// fn copy_file(
//     src: &Path,
//     dest: &PathBuf,
//     base_path: &PathBuf,
// ) -> Result<(), Box<dyn std::error::Error>> {
// use std::fs::{copy, create_dir_all};
// use std::path::{Path, PathBuf};
//     let relative_path = src.strip_prefix(base_path).unwrap();
//     let target_path = dest.join(relative_path);

//     if let Some(parent_directory) = target_path.parent() {
//         create_dir_all(parent_directory)?;
//     }

//     copy(src, target_path)?;
//     Ok(())
// }
