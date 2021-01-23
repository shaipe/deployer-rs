//! copyright © shaipe 2021 - present
//! 微服务应用内部命令处理
//! create by shaipe 202101023



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