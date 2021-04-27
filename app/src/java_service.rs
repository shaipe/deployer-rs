//! copyright © ecdata.cn 2021 - present
//! Java语言的服务启动安装
//! created by shaipe 20210427

/**

*/

pub struct JavaService {
    
}

impl JavaService {

    pub fn service_shell(name: &str){
        let shell_content = format!(
            r#"
#!/bin/bash

# 判断日志目录是否存，不存在创建目录
if [ ! -d "logs/" ];then
    mkdir logs
fi

# 启动应用，并给定输出日志目录
./orion_{name}.jar > logs/{name}-$(date +%Y%m%d%H%M%S).log
"#,
            name = name
        );
        // println!("{}\n{}", path, srv_content);
        // 把文件写入服务
        tube::fs::write_file("./start.sh", &shell_content.as_bytes());
    }
}