//! copyright © ecdata.cn 2021 - present
//! 本地文件上传到服务器
//! create by shaipe 20210103

use micro_app::Remote;
use reqwest::blocking::multipart;
use std::collections::HashMap;
use std::io::Read;
use std::path::Path;
use tube_error::Result;

/// 远程处理接口
pub trait RemoteService {
    /// 上传文件
    fn upload(
        &self,
        name: String,
        file_path: &Path,
        map: Option<HashMap<String, String>>,
    ) -> Result<String>;

    /// 调用远程命令
    fn call(&self, params: serde_json::Value) -> Result<Vec<String>>;
}

/// 远程调用业务实现
impl RemoteService for Remote {
    /// 文件上传
    fn upload(
        &self,
        name: String,
        file_path: &Path,
        map: Option<HashMap<String, String>>,
    ) -> Result<String> {
        // println!("url: {},  file_path: {:?}", name, file_path);

        let mut form = match multipart::Form::new().file(name, file_path) {
            Ok(f) => f,
            Err(err) => return Err(error!(format!("{:?}", err))),
        };

        if let Some(m) = map {
            for (k, v) in m {
                form = form.text(k, v);
            }
        };

        // Compose a request
        let client = reqwest::blocking::Client::new();
        let requestbuilder = client
            .post(&format!("{}/upload", self.get_url()))
            .timeout(std::time::Duration::from_secs(300))
            .multipart(form);

        // Send request
        let mut response = match requestbuilder.send() {
            Ok(res) => res,
            Err(err) => return Err(error!(format!("{:?}", err))),
        };

        if response.status().as_u16() == 200 {
            let mut response_data: Vec<u8> = vec![];
            match response.read_to_end(&mut response_data) {
                Ok(res) => res,
                Err(err) => return Err(error!(format!("{:?}", err))),
            };

            // let mut res_str = String::new();
            // response.read_to_string(mut res_str).unwrap();

            // let x: serde_json::Value = serde_json::from_reader(response_data).unwrap();

            // 获取返回的字符串
            if let Ok(res) = std::str::from_utf8(&response_data) {
                // 转换为json_value
                let val: serde_json::Value = match serde_json::from_str(res) {
                    Ok(res) => res,
                    Err(err) => return Err(error!(format!("{:?}", err))),
                };

                // 获取出上传后的相对路径
                if val["result"].is_object() {
                    if let Some(s) = val["result"]["relativePath"].as_str() {
                        return Ok(s.to_owned());
                    }
                }
            }
        }

        Err(error!("upload file failed"))
    }

    /// 调用远程命令
    fn call(&self, params: serde_json::Value) -> Result<Vec<String>> {
        let mut res = Vec::new();

        // Compose a request
        let client = reqwest::blocking::Client::new();
        let requestbuilder = client
            .post(&format!("{}/cmd", self.get_url()))
            .timeout(std::time::Duration::from_secs(300))
            .json(&params);

        // Send request
        let mut response = requestbuilder.send().unwrap();

        if response.status().as_u16() == 200 {
            let mut response_data: Vec<u8> = vec![];
            response.read_to_end(&mut response_data).unwrap();

            // 获取返回的字符串
            if let Ok(res_str) = std::str::from_utf8(&response_data) {
                // 转换为json_value
                let val: serde_json::Value = match serde_json::from_str(res_str) {
                    Ok(s) => s,
                    Err(err) => return Err(error!(format!("{:?}", err))),
                };

                // 获取出上传后的相对路径
                if let Some(arr) = val["result"].as_array() {
                    for a in arr {
                        if let Some(s) = a.as_str() {
                            res.push(s.to_owned());
                        }
                    }
                }
            }
        }

        Ok(res)
    }
}
// const URL: &'static str = "http://localhost:3000";
// pub fn upload_img() -> Result<(), reqwest::Error> {
//     let form = multipart::Form::new()
//         // Adding just a simple text field...
//         .text("username", "seanmonstar")
//         // And a file...
//         .file("photo", "photo.png")
//         .unwrap();

//     // Customize all the details of a Part if needed...
//     // let bio = multipart::Part::text("hallo peeps")
//     //     .file_name("sample.txt")
//     //     .mime_str("text/plain")
//     //     .unwrap();

//     // // Add the custom part to our form...
//     // let form = form.part("biography", bio);

//     // Compose a request
//     let client = reqwest::blocking::Client::new();
//     let requestbuilder = client.post(&String::from(URL)).multipart(form);

//     // Send request
//     let mut response = requestbuilder.send().unwrap();

//     // Report
//     println!("status: {}", response.status());
//     let mut response_data: Vec<u8> = vec![];
//     response.read_to_end(&mut response_data).unwrap();
//     println!(
//         "response:\n{}",
//         std::str::from_utf8(&response_data).unwrap()
//     );
//     Ok(())
// }
