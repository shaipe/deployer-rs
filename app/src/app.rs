//! copyright © shaipe 2021 - present
//! 微服务应用管理
//! create by shaipe 20210123

use super::service;
use serde::{Deserialize, Serialize};
use tube_error::Result;

/// 应用
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct App {
    // 应用项目代号
    pub symbol: String,
    // 微服务名称
    pub name: String,
    // 标题
    pub description: String,
    // 应用工作目录
    pub workdir: String,
    // 端口
    pub port: u16,
    // 应用状态
    pub status: u16,
    // 当前版本
    pub version: String,
    // 应用开发语言
    pub lang: String,
}

impl App {
    /// 新建应用对象
    pub fn new(base_dir: &str, symbol: &str, name: &str) -> App {
        App {
            symbol: symbol.to_owned(),
            name: name.to_owned(),
            description: "".to_owned(),
            workdir: format!(
                "{workdir}/{symbol}/apps/{name}",
                workdir = base_dir,
                symbol = symbol,
                name = name
            ),
            port: 7000,
            status: 0,
            version: "0.1.0".to_owned(),
            lang: "java".to_owned(),
        }
    }

    /// 安装服务
    pub fn install_service(&self) -> Result<bool> {
        let srv_name = format!("{}_{}", self.symbol, self.name);
        // println!("srv name {}", srv_name);
        if self.lang == "java" {
            match self.install_start_shell() {
                Ok(s) => {
                    let cmd = format!("{} &", s);
                    let srv = service::Service::new(&srv_name, &cmd, 180);
                    srv.install()
                }
                Err(err) => Err(err),
            }
        } else {
            let cmd = format!("{}/{} &", self.workdir, srv_name);
            let srv = service::Service::new(&srv_name, &cmd, 180);
            srv.install()
        }
    }

    /// 获取jar应用名
    pub fn app_name(&self) -> String {
        if self.symbol.len()>0{
            format!("{}_{}{}", self.symbol, self.name, self.get_ext())
        }
        else{
            format!("{}{}", self.name, self.get_ext())
        }
    }

    /// 获取应用包的扩展名
    fn get_ext(&self) -> &'static str {
        if &self.lang == "java" {
            ".jar"
        } else {
            if cfg!(target_os = "windows") {
                ".exe"
            } else if cfg!(target_os = "macos") {
                ".app"
            } else {
                ""
            }
        }
    }

    /// 获取java包启动脚本
    /// springboot应用程序部署脚本
    /// 在使用maven进持package打包时，修改应用的pom文件，
    /// <plugin>
    ///     <groupId>org.springframework.boot</groupId>
    ///   <artifactId>spring-boot-maven-plugin</artifactId>
    ///   <configuration>
    ///     <!-- 使用此配置之后打包的jar,可以直接使用./xxx.jar启动执行-->
    ///     <executable>true</executable>
    ///   </configuration>
    /// </plugin>
    pub fn install_start_shell(&self) -> Result<String> {
        if cfg!(feature = "java") {
            let content = format!(
                r#"#!/bin/bash
                if [ -f "./log/{name}.log" ];then
                    # 获取当前时间
                    current=$(date +%Y%m%d%H%M%S)
                    # 备份之前日志
                    mv ./log/{name}.log ./log/$current_{name}.log
                fi
                # 启动应用
                {dir}/{app_name}.jar > {dir}/log/{name}.log &"#,
                app_name = self.app_name(),
                name = self.name,
                dir = self.workdir
            );

            let start_path = format!("{}/start.sh", self.workdir);

            tube::fs::write_file(&start_path, &content.as_bytes());
            return Ok(start_path);
        };
        Ok("".to_owned())
    }
}
