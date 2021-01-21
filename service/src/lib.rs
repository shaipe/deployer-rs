//! copyright © shaipe 2021 - present
//! 系统服务
//! create by shaipe 202101021

mod linux;

/// 服务安装到linux
pub fn install_linux(name: &str, cmd: &str, timeout: u16) {
    let path = format!("/lib/systemd/system/{name}.service", name=name);
    let serv = linux::Service::new(name, cmd, timeout);
    tube::fs::write_file(&path, &serv.to_linux().as_bytes());
}


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
