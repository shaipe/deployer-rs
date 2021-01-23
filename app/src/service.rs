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
        }
    }

    /// 服务安装
    pub fn install(&self) -> Result<bool> {
        use super::cmd::run_cmd;
        if cfg!(target_os = "linux") {
            let path = format!("/lib/systemd/system/{name}.service", name = self.name);
            let srv_content = format!(
                r#"
        [Unit]
        Description={name}
        After=network.target
        
        [Service]
        Type=forking
        ExecStart={cmd}
        PrivateTmp=true
        TimeoutStartSec={timeout}
        
        [Install]
        WantedBy=multi-user.target
        "#,
                timeout = self.timeout,
                cmd = self.command,
                name = self.name
            );
            // 把文件写入服务
            tube::fs::write_file(&path, &srv_content.as_bytes());
            // 设置应用为自启动
            if let Ok(_r) = run_cmd(&format!("systemctl enable {}", self.name), "./", true) {
                return Ok(true);
            }
        } else if cfg!(target_os = "windows") {
            println!("Hello Windows");
        } else {
            println!("Unknown os");
        }
        Ok(false)
    }
}
