//! copyright © ecdata.cn 2021 - present
//! 系统服务
//! create by shaipe 202101021


#[macro_use]
extern crate tube_error;
// 在主文件中必须要引入Error类型,来定义整个包的基础错误类型
use tube_error::Error;


mod service;
mod cmd;
mod app;
mod pool;
mod docker;
mod remote;
mod java_service;

pub use service::Service;
pub use app::App;
pub use pool::Pool;
pub use docker::Docker;
pub use remote::Remote;
pub use java_service::JavaService;


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
