//! copyright © shaipe 2021 - present
//! Linux 服务处理
//! linux 服务存放位置 /lib/systemd/system/{name}.service
//! create by shaipe 202101021

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
impl Service {
    /// 新建一个服务结构体
    pub fn new(name: &str, cmd: &str, timeout: u16) -> Self {
        Service {
            name: name.to_owned(),
            timeout: timeout,
            command: cmd.to_owned(),
        }
    }

    /// 转换为linux服务配置
    pub fn to_linux(&self) -> String {
        format!(
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
        )
    }
}
