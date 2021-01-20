//! copyright © shaipe 2021 - present
//! 命令行操作应用类
//! create by shaipe 20210102

use tube_cmd::Command;
use tube_error::Result;

/// 运行命令
pub fn run_cmd(cmd: &str, env_dir: &str, enable_capture: bool) -> Result<String> {
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

// fn test() {
//     // let pwd = Command::new("pwd").output().expect("/");

//     // let output = if cfg!(target_os = "windows") {
//     //     Command::new("cmd")
//     //             .args(&["/C", "echo hello"])
//     //             .output()
//     //             .expect("failed to execute process")
//     // } else {
//     //     Command::new("sh")
//     //             .arg("-c")
//     //             .arg("echo hello")
//     //             .output()
//     //             .expect("failed to execute process")
//     // };

//     // let hello = pwd.stdout;

//     let x = Command::new("ls")
//         .current_dir("./cli")
//         .spawn()
//         .expect("ls command failed to start");

//     let hello = x.wait_with_output().unwrap().stdout;

//     // String::from_utf8(output.stdout)?
//     //     .lines()
//     //     .filter_map(|line| pattern.captures(line))
//     //     .map(|cap| {
//     //              Commit {
//     //                  hash: cap[1].to_string(),
//     //                  message: cap[2].trim().to_string(),
//     //              }
//     //          })
//     //     .take(5)
//     //     .for_each(|x| println!("{:?}", x));

//     println!("{:?}", std::str::from_utf8(&hello).unwrap());
// }
