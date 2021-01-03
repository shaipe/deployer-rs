//! copyright © shaipe 2021 - present
//! 服务部署器客户端工具
//! create by shaipe 20210102

use clap::{crate_authors, crate_description, crate_version, App, AppSettings, Arg};
use std::error::Error;


mod config;
use config::Config;

// mod upload;
// use upload::upload_file;

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
        .subcommand(App::new("test")
            .about("controls testing features")
            .version("1.3")
            .author("Someone E. <someone_else@other.com>")
            .arg(Arg::new("debug")
                .short('d')
                .about("print debug information verbosely")))
        .get_matches();

    let conf_path = matches.value_of("config").unwrap_or("conf/cli.yml");
    println!("Value for config: {}", conf_path);

    let c = match Config::new(conf_path) {
        Ok(c) => c,
        Err(e) => panic!("ddd"),
    };

    println!("c::{:?}", c);

    // upload_file();

    // match matches.values_of("name") {
    //     None => {}
    //     Some(val) => {
    //         println!("{:?}", val);
    //     }
    // }

    Ok(())
}
