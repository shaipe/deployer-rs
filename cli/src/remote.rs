//! copyright © shaipe 2021 - present
//! 本地文件上传到服务器
//! create by shaipe 20210103

use reqwest::blocking::multipart;
use std::collections::HashMap;
use std::io::Read;
use std::path::Path;

/// 文件上传
pub fn upload_file(
    uri: &str,
    name: String,
    file_path: &Path,
    map: Option<HashMap<String, String>>,
) -> Result<(), reqwest::Error> {
    println!("url: {}, name: {}, file_path: {:?}", uri, name, file_path);
    let mut form = multipart::Form::new().file(name, file_path).unwrap();

    if let Some(m) = map {
        for (k, v) in m {
            form = form.text(k, v);
        }
    };

    // let form = multipart::Form::new()
    //     // Adding just a simple text field...
    //     .text("username", "seanmonstar")
    //     // And a file...
    //     .file("photo", "photo.png")
    //     .unwrap();

    // Customize all the details of a Part if needed...
    // let bio = multipart::Part::text("hallo peeps")
    //     .file_name("sample.txt")
    //     .mime_str("text/plain")
    //     .unwrap();

    // // Add the custom part to our form...
    // let form = form.part("biography", bio);

    // Compose a request
    let client = reqwest::blocking::Client::new();
    let requestbuilder = client.post(&String::from(uri)).multipart(form);

    // Send request
    let mut response = requestbuilder.send().unwrap();

    // Report
    println!("status: {}", response.status());
    let mut response_data: Vec<u8> = vec![];
    response.read_to_end(&mut response_data).unwrap();
    println!(
        "response:\n{}",
        std::str::from_utf8(&response_data).unwrap()
    );
    Ok(())
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
