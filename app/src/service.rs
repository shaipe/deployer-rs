//! copyright © shaipe 2021 - present
//! Linux 服务处理
//! linux 服务存放位置 /lib/systemd/system/{name}.service
//! create by shaipe 202101021

use super::cmd::run_cmd;
use serde::{Deserialize, Serialize};
use tube_error::Result;

/// 服务结构体
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Service {
    // 服务描述名称
    pub name: String,
    // 服务启动超时时间
    pub timeout: u16,
    // 服务的工作目录
    pub workdir: String,
    // 启动执行命令
    pub exec: String,
    // 启动参数
    pub args: Option<String>,
}

/// 服务方法实现
// #[cfg(target_os = "linux")]
impl Service {
    /// 新建一个服务结构体
    pub fn new(workdir: &str, name: &str, cmd: &str, timeout: u16) -> Self {
        Service {
            name: name.to_owned(),
            timeout: timeout,
            workdir: workdir.to_owned(),
            exec: cmd.to_owned(),
            args: None,
        }
    }

    /// 服务安装
    pub fn install(&self) -> Result<Vec<String>> {
        let mut res: Vec<String> = Vec::new();
        if cfg!(target_os = "linux") {
            let mut cmd = self.exec.clone();
            if let Some(arg) = self.args.clone() {
                cmd = format!("{} {}", self.exec, arg);
            }
            match Service::install_linux_service(
                &self.workdir,
                &self.name,
                &cmd,
                self.timeout,
            ){
                Ok(s)=>res.push(s),
                Err(err) => res.push(format!("error: {}", err)),
            };
        } else if cfg!(target_os = "windows") {
            res.push("Hello Windows".to_owned());
        } else {
            res.push("Unknown os".to_owned());
        }
        Ok(res)
    }

    /// 应用程序备份
    pub fn unzip(&self, zip_file: &str) -> Result<Vec<String>> {
        let mut res: Vec<String> = Vec::new();
        // 对服务进行应用和日志备份
        
        match tube::unzip(zip_file, &self.workdir) {
            Ok(_) => res.push("unzip file successfully".to_owned()),
            Err(err) => res.push(format!("error:{:?}", err)),
        };

        Ok(res)
    }

    // /// 获取java包启动脚本
    // /// springboot应用程序部署脚本
    // /// 在使用maven进持package打包时，修改应用的pom文件，
    // /// <plugin>
    // ///     <groupId>org.springframework.boot</groupId>
    // ///   <artifactId>spring-boot-maven-plugin</artifactId>
    // ///   <configuration>
    // ///     <!-- 使用此配置之后打包的jar,可以直接使用./xxx.jar启动执行-->
    // ///     <executable>true</executable>
    // ///   </configuration>
    // /// </plugin>
    // pub fn install_start_shell(&self) -> Result<String> {
    //     if cfg!(feature = "java") {
    //         let content = format!(
    //             r#"#!/bin/bash
    //             if [ -f "./log/{name}.log" ];then
    //                 # 获取当前时间
    //                 current=$(date +%Y%m%d%H%M%S)
    //                 # 备份之前日志
    //                 mv ./log/{name}.log ./log/$current_{name}.log
    //             fi
    //             # 启动应用
    //             {dir}/{app_name}.jar > {dir}/log/{name}.log &"#,
    //             app_name = self.app_name(),
    //             name = self.name,
    //             dir = self.workdir
    //         );

    //         let start_path = format!("{}/start.sh", self.workdir);

    //         tube::fs::write_file(&start_path, &content.as_bytes());
    //         return Ok(start_path);
    //     };
    //     Ok("".to_owned())
    // }

    /// 应用程序备份
    pub fn backup(&self) -> Result<Vec<String>> {
        let mut res: Vec<String> = Vec::new();
        // 对服务进行应用和日志备份

        let bak_path = format!("{}.bak", self.exec);
        match std::fs::copy(&self.exec, bak_path) {
            Ok(_) => res.push("backup app file successfully".to_owned()),
            Err(err) => res.push(format!("error:{:?}", err)),
        };

        // let log_path = format!("{}/{}.log", self.workdir, self.name);
        // let bak_log_path = format!("{}/log/{}_{}.log", self.workdir, self.name, 333);
        // match std::fs::copy(log_path, bak_log_path) {
        //     Ok(_) => res.push("backup log file successfully".to_owned()),
        //     Err(err) => res.push(format!("error:{:?}", err)),
        // }

        Ok(res)
    }

    // pub fn uninstall(&self) -> Result<bool> {
    //     match Service::stop(&self.name) {
    //         Ok(_) => {
    //             std::fs::remove_file(format!(
    //                 "/lib/systemd/system/{name}.service",
    //                 name = self.name
    //             ));
    //         }
    //         Err(err) => Err(err),
    //     }
    // }

    /// Linuxt系统服务安装
    pub fn install_linux_service(
        workdir: &str,
        name: &str,
        cmd: &str,
        timeout: u16,
    ) -> Result<String> {
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

        if let Ok(_r) = run_cmd(&format!("systemctl enable {}", name), "", true) {
            return Ok("install service successfully!".to_owned());
        }
        Err(error!("install service failed"))
    }

    /// 启动服务
    pub fn start(name: &str) -> Result<bool> {
        if let Ok(_r) = run_cmd(&format!("systemctl start {}", name), "", true) {
            return Ok(true);
        }
        Err(error!("start failed"))
    }

    /// 停止服务
    pub fn stop(name: &str) -> Result<bool> {
        if let Ok(_r) = run_cmd(&format!("systemctl stop {}", name), "", true) {
            return Ok(true);
        }
        Err(error!("stop failed"))
    }

    /// 重启动服务
    pub fn restart(name: &str) -> Result<bool> {
        if let Ok(_r) = run_cmd(&format!("systemctl restart {}", name), "", true) {
            return Ok(true);
        }
        Err(error!("restart failed"))
    }

    /// 获取服务状态
    pub fn status(name: &str) -> Result<Vec<String>> {
        if let Ok(r) = run_cmd(&format!("systemctl status {} -l", name), "", true) {
            return Ok(r);
        }
        Err(error!("status failed"))
    }
}
