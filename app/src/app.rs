//! copyright © shaipe 2021 - present
//! 微服务应用管理
//! create by shaipe 20210123

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
}

impl App {
    /// 新建应用对象
    pub fn new(name: String) -> App {
        App {
            symbol: "".to_owned(),
            name,
            description: "".to_owned(),
            workdir: "./".to_owned(),
            port: 7000,
            status: 0,
            version: "".to_owned(),
        }
    }

    pub fn install_service(&self) {}

    /// 获取jar应用名
    pub fn app_name(&self) -> String {
        if cfg!(feature = "java") {
            format!("{}_{}-{}.jar", self.symbol, self.name, self.version)
        } else {
            format!("{}_{}-{}", self.symbol, self.name, self.version)
        }
    }

    /// 获取java包启动脚本
    /// # springboot应用程序部署脚本
    /// # 在使用maven进持package打包时，修改应用的pom文件，
    /// # <plugin>
    /// #   <groupId>org.springframework.boot</groupId>
    /// #   <artifactId>spring-boot-maven-plugin</artifactId>
    /// #   <configuration>
    /// #     <!-- 使用此配置之后打包的jar,可以直接使用./xxx.jar启动执行-->
    /// #     <executable>true</executable>
    /// #   </configuration>
    /// # </plugin>
    pub fn install_start_shell(&self, base_dir: &str) -> Result<String> {
        if cfg!(feature = "java") {
            let app_dir = format!(
                "{workdir}/{symbol}/apps/{name}",
                workdir = base_dir,
                symbol = self.symbol,
                name = self.name
            );
            let content = format!(
                r#"#!/bin/bash
        
        {dir}/{app_name}.jar > {dir}/log/{name}.log &"#,
                app_name = self.app_name(),
                name = self.name,
                dir = app_dir
            );

            let start_path = format!("{}/start.sh", app_dir);

            tube::fs::write_file(&start_path, &content.as_bytes());
            return Ok(start_path);
        } else {
        };
        Ok("".to_owned())
    }
}
