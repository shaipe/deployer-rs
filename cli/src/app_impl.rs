//! copyright © shaipe 2021 - present
//! 命令行操作应用类
//! create by shaipe 20210102

use crate::config::App;
use crate::RemoteImpl;
use tube_error::Result;

/// 对app应用处理
pub trait AppImpl {
    fn install(&self) -> Result<Vec<String>>;

    fn update(&self) -> Result<Vec<String>>;

    fn start(&self) -> Result<Vec<String>>;

    fn remote(&self, action: &str) -> Result<Vec<String>>;

    fn end(&self) -> Result<Vec<String>>;
}

impl AppImpl for App {
    /// 开始命令执行
    fn install(&self) -> Result<Vec<String>> {
        use std::path::Path;
        // 1. 复制并上传文件
        let f_str = format!("{}/{}.zip", self.workdir, self.name);
        let f_path = Path::new(&f_str);
        let name = f_path.file_stem().unwrap().to_str().unwrap();
        let up_res = self.remote.upload(name.to_owned(), f_path, None);

        // 2. 调用执行远端命令
        if let Ok(relative_path) = up_res {
            if relative_path.len() > 0 {
                let yy = self.remote.call(serde_json::json!({
                    "workdir": self.workdir,
                    "action": "install",
                    "filePath": relative_path,
                    "app": {
                        "symbol": self.symbol,
                        "name": self.name,
                        "version": self.version,
                        "isServer": self.is_service,
                        // "execStart": self.exec_start
                    },
                    "start": self.start,
                    "end": self.end
                }));
                println!("{:?}", yy);
            }
        }
        Ok(vec![])
    }

    /// 开始命令执行
    fn update(&self) -> Result<Vec<String>> {
        let mut res: Vec<String> = Vec::new();
        for cmd in &self.start {
            if let Ok(x) = run_cmd(cmd, &self.workdir, true) {
                res.extend(x);
            }
        }
        Ok(res)
    }

    /// 开始命令执行
    fn start(&self) -> Result<Vec<String>> {
        let mut res: Vec<String> = Vec::new();
        for cmd in &self.start {
            if let Ok(x) = run_cmd(cmd, &self.workdir, true) {
                res.extend(x);
            }
        }
        Ok(res)
    }

    /// 远程相关处理
    fn remote(&self, action: &str) -> Result<Vec<String>> {
        use std::path::Path;
        // 1. 复制并上传文件
        let f_str = format!("{}/{}.zip", self.workdir, self.name);
        let f_path = Path::new(&f_str);
        let name = f_path.file_stem().unwrap().to_str().unwrap();
        let up_res = self.remote.upload(name.to_owned(), f_path, None);

        // 2. 调用执行远端命令
        if let Ok(relative_path) = up_res {
            if relative_path.len() > 0 {
                let yy = self.remote.call(serde_json::json!({
                    "workdir": self.workdir,
                    "action": action,
                    "data": {
                        "relativePath": relative_path
                    },
                    "start": self.start,
                    "end": self.end
                }));
                println!("{:?}", yy);
            }
        }
        Ok(vec![])
    }

    /// 执行完后的处理
    fn end(&self) -> Result<Vec<String>> {
        let mut res: Vec<String> = Vec::new();
        for cmd in &self.end {
            if let Ok(x) = run_cmd(cmd, &self.workdir, true) {
                res.extend(x);
            }
        }
        Ok(res)
    }
}

/// 运行命令
pub(crate) fn run_cmd(
    cmd: &str,
    env_dir: &str,
    enable_capture: bool,
) -> tube_error::Result<Vec<String>> {
    use tube_cmd::Command;
    // let cmd = Command::with_args("bash", &["-c", "ls ; sleep 2; ls"]).set_dir(env_dir).add_args(&[cmd]);
    // 对操作系统进行判断
    let cmd_name = if cfg!(target_os = "Windows") {
        "ps"
    } else {
        "bash"
    };

    let res = if enable_capture {
        Command::with_args(cmd_name, &["-c", cmd])
            .set_dir(env_dir.clone())
            .enable_capture()
            .run()
    } else {
        Command::with_args(cmd_name, &["-c", cmd])
            .set_dir(env_dir.clone())
            .run()
    };

    let res = match res {
        Ok(s) => format!("{}", s.stdout_string_lossy()),
        Err(e) => {
            // println!("{:?}", e);
            format!("{:?}", e.to_string())
        }
    };
    let x = res.lines().map(|x| x.to_owned()).collect::<Vec<String>>();
    Ok(x)
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
