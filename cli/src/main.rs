//! copyright © shaipe 2021 - present
//! 服务部署器客户端工具
//! create by shaipe 20210102
#[macro_use]
extern crate tube_error;

// 在主文件中必须要引入Error类型,来定义整个包的基础错误类型
use tube_error::Error;

use clap::{crate_authors, crate_description, crate_version, App, Arg};

mod config;
use config::Config;

mod remote;
use remote::upload_file;

mod cmd;

use std::fs::{copy, create_dir_all};
use std::path::{Path, PathBuf};



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
            App::new("test")
                .about("controls testing features")
                .version("1.3")
                .author("Someone E. <someone_else@other.com>")
                .arg(Arg::with_name("debug").short("d")),
        )
        .get_matches();

    // 加载配置文件
    let conf_path = matches.value_of("config").unwrap_or("conf/cli.yml");

    println!("Value for config: {}", conf_path);

    let cnf = match Config::new(conf_path) {
        Ok(c) => c,
        Err(_e) => panic!("配置文件加载错误."),
    };

    println!("c::{:?}", cnf);

    // 1. 开始执行的命令
    // for lc in cnf.local.start_cmd {
    //     println!("cmd::{}", lc);
    //     let cmd_res = cmd::run_cmd(&lc, &cnf.local.workdir, false);
    //     println!("{:?}", cmd_res);
    // }

    // 2. 复制并上传文件
    // remote::upload_img();
    let f_str = format!("{}/{}", cnf.local.workdir, cnf.local.upload_file);
    let f_path = Path::new(&f_str);
    let name = f_path.file_stem().unwrap().to_str().unwrap();
    let up_res = upload_file(&cnf.local.upload_url, name.to_owned(), f_path, None);
    println!("{:?}", up_res);

    // 3. 调用执行远端命令

    // 4. 完成后执行的本地命令
    for lc in cnf.local.end_cmd {
        println!("cmd::{}", lc);
        let cmd_res = cmd::run_cmd(&lc, &cnf.local.workdir, false);
        println!("{:?}", cmd_res);
    }

    println!("自动更新命令完成...");

    Ok(())
}


// fn copy_file(
//     src: &Path,
//     dest: &PathBuf,
//     base_path: &PathBuf,
// ) -> Result<(), Box<dyn std::error::Error>> {
//     let relative_path = src.strip_prefix(base_path).unwrap();
//     let target_path = dest.join(relative_path);

//     if let Some(parent_directory) = target_path.parent() {
//         create_dir_all(parent_directory)?;
//     }

//     copy(src, target_path)?;
//     Ok(())
// }