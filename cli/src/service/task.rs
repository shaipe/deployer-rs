//! copyright © shaipe 2021 - present
//! 命令行操作应用类
//! create by shaipe 20210102

use super::RemoteService;
use crate::config::Task;
use tube_error::Result;

/// 对app应用处理
pub trait TaskService {
    /// 安装应用
    fn install(&self) -> Result<Vec<String>>;
    /// 更新应用任务
    fn update(&self) -> Result<Vec<String>>;

    fn start(&self) -> Result<Vec<String>>;

    fn remote(&self, action: &str) -> Result<Vec<String>>;

    fn end(&self) -> Result<Vec<String>>;
}

impl TaskService for Task {
    /// 开始命令执行
    fn install(&self) -> Result<Vec<String>> {
        let mut res: Vec<String> = Vec::new();
        // 1. 本地打包
        match self.start() {
            Ok(s) => {
                res.extend(s);
                // 2. 上传并进行远程处理
                match self.remote("install") {
                    Ok(rs) => {
                        res.extend(rs);
                        match self.end() {
                            Ok(es) => {
                                res.extend(es);
                            }
                            Err(err) => res.push(format!("error:{}", err)),
                        }
                    }
                    Err(err) => res.push(format!("error:{}", err)),
                }
            }
            Err(err) => res.push(format!("error:{}", err)),
        }

        Ok(res)
    }

    /// 开始命令执行
    fn update(&self) -> Result<Vec<String>> {
        let mut res: Vec<String> = Vec::new();
        // 1. 本地打包
        match self.start() {
            Ok(s) => {
                res.extend(s);
                // 2. 上传并进行远程处理
                match self.remote("update") {
                    Ok(rs) => {
                        res.extend(rs);
                        match self.end() {
                            Ok(es) => {
                                res.extend(es);
                            }
                            Err(err) => res.push(format!("error:{}", err)),
                        }
                    }
                    Err(err) => res.push(format!("error:{}", err)),
                }
            }
            Err(err) => res.push(format!("error:{}", err)),
        }

        Ok(res)
    }

    /// 开始命令执行
    fn start(&self) -> Result<Vec<String>> {
        let mut res: Vec<String> = Vec::new();
        for cmd in &self.start {
            let c = cmd
                .replace("$symbol", &self.symbol)
                .replace("$version", &self.version)
                .replace("$name", &self.name);
            if let Ok(x) = run_cmd(&c, &self.app.code_dir, false) {
                res.extend(x);
            }
        }
        Ok(res)
    }

    /// 远程相关处理

    fn remote(&self, action: &str) -> Result<Vec<String>> {
        use std::path::Path;
        let mut res = Vec::new();
        if let Some(remote) = self.remote.clone() {
            // 1. 复制并上传文件
            let f_str = format!("{}/{}.zip", self.app.code_dir, self.name);
            let f_path = Path::new(&f_str);
            if !f_path.exists() {
                return Err(error!("upload file not found"));
            }
            let name = f_path.file_stem().unwrap().to_str().unwrap();
            println!("start upload file...");
            let up_res = remote.upload(name.to_owned(), f_path, None);

            // 2. 调用执行远端命令
            match up_res {
                Ok(relative_path) => {
                    if relative_path.len() > 0 {
                        res.push(format!("upload file success path : {:?}", relative_path));
                        let cmd_res = remote.call(serde_json::json!({
                            "symbol": self.symbol,
                            "name": self.name,
                            "appType": self.app_type,
                            "service": self.service,
                            // "docker": Option<Docker>,
                            "workdir": remote.workdir,
                            "action": action,
                            "filePath": relative_path,
                            // "app": self.app, 远程不再接收
                            "start": remote.start,
                            "end": remote.end
                        }));
                        match cmd_res {
                            Ok(s) => {
                                res.extend(s);
                            }
                            Err(err) => println!("remote call error: {}", err),
                        }
                    } else {
                        res.push("upload file failed!".to_owned());
                    }
                }
                Err(err) => res.push(format!("error:{}", err)),
            }
        }
        Ok(res)
    }

    /// 执行完后的处理
    fn end(&self) -> Result<Vec<String>> {
        let mut res: Vec<String> = Vec::new();
        for cmd in &self.end {
            let c = cmd
                .replace("$symbol", &self.symbol)
                .replace("$name", &self.name);
            if let Ok(x) = run_cmd(&c, &self.app.code_dir, false) {
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
    use tube::cmd::Command;
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
