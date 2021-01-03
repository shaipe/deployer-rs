//! copyright © shaipe 2021 - present
//! 服务部署器客户端工具
//! create by shaipe 20210102

use clap::{crate_authors, crate_description, crate_version, App, AppSettings, Arg};
use std::error::Error;

// #[macro_use]
// extern crate clap;
mod config;
use config::Config;

fn main() -> Result<(), Box<dyn Error>> {
    let matches = App::new("dcli")
        .version(crate_version!())
        .author(crate_authors!())
        .about(crate_description!())
        .setting(AppSettings::SubcommandRequiredElseHelp)
        .arg(
            Arg::new("config")
                .short('c')
                .long("config")
                .value_name("FILE")
                // .help("set name of program")
                .takes_value(true),
        )
        // .subcommands(vec![
        //     SubCommand::new("init")
        //         .about("Create a new Zola project")
        //         .args(&[
        //             Arg::new("name")
        //                 .default_value(".")
        //                 // .help("Name of the project. Will create a new directory with that name in the current directory"),
        //             Arg::new("force")
        //                 .short("f")
        //                 .takes_value(false)
        //                 // .help("Force creation of project even if directory is non-empty")
        //         ])
        // ])
        .get_matches();

    let conf_path = matches.value_of("config").unwrap_or("conf/cli.yml");
    println!("Value for config: {}", conf_path);

    let c = match Config::new(conf_path) {
        Ok(c) => c,
        Err(e) => panic!("ddd"),
    };

    println!("c::{:?}", c);

    // match matches.values_of("name") {
    //     None => {}
    //     Some(val) => {
    //         println!("{:?}", val);
    //     }
    // }

    Ok(())
}
