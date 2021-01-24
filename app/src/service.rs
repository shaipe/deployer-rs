//! copyright © shaipe 2021 - present
//! Linux 服务处理
//! linux 服务存放位置 /lib/systemd/system/{name}.service
//! create by shaipe 202101021

use tube_error::Result;

/// 服务结构体
#[derive(Debug, Clone)]
pub struct Service {
    // 服务描述名称
    pub name: String,
    // 服务启动超时时间
    pub timeout: u16,
    // 服务的工作目录
    pub workdir: String,
    // 启动执行命令
    pub command: String,
}

/// 服务方法实现
// #[cfg(target_os = "linux")]
impl Service {
    /// 新建一个服务结构体
    pub fn new(name: &str, cmd: &str, timeout: u16) -> Self {
        Service {
            name: name.to_owned(),
            timeout: timeout,
            command: cmd.to_owned(),
            workdir: "/".to_owned(),
        }
    }

    /// 服务安装
    pub fn install(&self) -> Result<bool> {
        if cfg!(target_os = "linux") {
            return Service::install_linux_service(
                &self.workdir,
                &self.name,
                &self.command,
                self.timeout,
            );
            // return Ok(true);
        } else if cfg!(target_os = "windows") {
            println!("Hello Windows");
        } else {
            println!("Unknown os");
        }
        Ok(false)
    }

    /// Linuxt系统服务安装
    pub fn install_linux_service(
        workdir: &str,
        name: &str,
        cmd: &str,
        timeout: u16,
    ) -> Result<bool> {
        let path = format!("/lib/systemd/system/{name}.service", name = name);
        let srv_content = format!(
            r#"
[Unit]
Description={name}
After=network.target

[Service]
Type=simple
WorkingDirectory={workdir}
ExecStart={cmd}
# PrivateTmp=true
# TimeoutStartSec={timeout}

[Install]
WantedBy=multi-user.target
"#,
            workdir = workdir,
            timeout = timeout,
            cmd = cmd,
            name = name
        );
        // println!("{}\n{}", path, srv_content);
        // 把文件写入服务
        tube::fs::write_file(&path, &srv_content.as_bytes());
        // 设置应用为自启动
        use super::cmd::run_cmd;
        if let Ok(_r) = run_cmd(&format!("systemctl enable {}", name), "", true) {
            return Ok(true);
        }
        Ok(false)
    }

    /// 启动服务
    pub fn start(name: &str) -> Result<bool> {
        use super::cmd::run_cmd;
        if let Ok(_r) = run_cmd(&format!("systemctl start {}", name), "", true) {
            return Ok(true);
        }
        Ok(false)
    }
}
