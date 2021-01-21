//! copyright © shaipe 2021 - present
//! 系统服务
//! create by shaipe 202101021

mod linux;

/// 服务安装到linux
pub fn install_linux(name: &str, cmd: &str, timeout: u16) {
    let path = format!("/lib/systemd/system/{name}.service", name = name);
    let serv = linux::Service::new(name, cmd, timeout);
    tube::fs::write_file(&path, &serv.to_linux().as_bytes());
}

/// 微服务应用管理
pub struct App {
    // 应用项目代号
    pub symbol: String,
    // 微服务名称
    pub name: String,
    // 当前版本
    pub version: String,
}

impl App {
    pub fn new() -> Self {
        App {
            symbol: "hawk".to_owned(),
            name: "".to_owned(),
            version: "0.1.0".to_owned(),
        }
    }

    pub fn app_jar_name(&self) -> String {
        format!("{}_{}-{}.jar", self.symbol, self.name, self.version)
    }

    /// 获取启动脚本
    pub fn get_jar_start_shell(&self, base_dir: &str) -> String {
        let app_dir = format!(
            "{workdir}/{symbol}/apps/{name}",
            workdir = base_dir,
            symbol = self.symbol,
            name = self.name
        );
        format!(
            r#"#!/bin/bash
        
        {dir}/{app_name}.jar > {dir}/{name}.log &"#,
            app_name = self.app_jar_name(),
            name = self.name,
            dir = app_dir
        )
    }
}

/// 写入启动程序脚本
pub fn write_start(workdir: &str, name: &str, version: &str) {
    let content = format!(
        r#"#!/bin/bash
    
    {workdir}/{name}-{version}.jar > {workdir}/{name}.log &"#,
        name = name,
        workdir = workdir,
        version = version
    );

    tube::fs::write_file(&format!("{}/start.sh", workdir), &content.as_bytes());
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
