//! copyright © shaipe 2021 - present
//! 服务部署器客户端工具
//! create by shaipe 20210102


use clap::{App, Arg};
use std::error::Error;

// #[macro_use]
// extern crate clap;
mod config;
use config::Config;

fn main() -> Result<(), Box<dyn Error>> {

    // let yaml = load_yaml!("../cli.yml");
    // let matches = App::from_yaml(yaml).get_matches();

    // println!("{:?}", matches);

    let matches = App::new("dcli")
        .version(clap::crate_version!())
        .author("shaipe")
        .about("deployer of client program")
        .arg(
            Arg::with_name("config")
                .short("c")
                .long("config")
                .value_name("FILE")
                .help("set name of program")
                .takes_value(true),
        )
        .get_matches();


    let conf_path = matches.value_of("config").unwrap_or("conf/cli.yml");
    println!("Value for config: {}", conf_path);

    let c = match Config::new(conf_path){
        Ok(c) => c,
        Err(e) => panic!("ddd")
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
