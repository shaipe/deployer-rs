//! copyright © shaipe 2021 - present
//! 命令行操作应用类
//! create by shaipe 20210102

use std::process::Command;

fn test(){

    // let pwd = Command::new("pwd").output().expect("/");

    // let output = if cfg!(target_os = "windows") {
    //     Command::new("cmd")
    //             .args(&["/C", "echo hello"])
    //             .output()
    //             .expect("failed to execute process")
    // } else {
    //     Command::new("sh")
    //             .arg("-c")
    //             .arg("echo hello")
    //             .output()
    //             .expect("failed to execute process")
    // };
    
    // let hello = pwd.stdout;

    let x = Command::new("ls")
        .current_dir("./cli")
        .spawn()
        .expect("ls command failed to start");
    
    let hello = x.wait_with_output().unwrap().stdout;

    String::from_utf8(output.stdout)?
        .lines()
        .filter_map(|line| pattern.captures(line))
        .map(|cap| {
                 Commit {
                     hash: cap[1].to_string(),
                     message: cap[2].trim().to_string(),
                 }
             })
        .take(5)
        .for_each(|x| println!("{:?}", x));

    println!("{:?}", std::str::from_utf8(&hello).unwrap());
}

fn run_cmd() {
    
}