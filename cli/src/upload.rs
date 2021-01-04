//! copyright © shaipe 2021 - present
//! 本地文件上传到服务器
//! create by shaipe 20210103

use reqwest::blocking::multipart;
use std::io::Read;

const URL: &'static str = "http://localhost:3000";
pub fn upload_file() -> Result<(), reqwest::Error> {
    let form = multipart::Form::new()
        // Adding just a simple text field...
        .text("username", "seanmonstar")
        // And a file...
        .file("photo", "photo.png")
        .unwrap();

    // Customize all the details of a Part if needed...
    // let bio = multipart::Part::text("hallo peeps")
    //     .file_name("sample.txt")
    //     .mime_str("text/plain")
    //     .unwrap();

    // // Add the custom part to our form...
    // let form = form.part("biography", bio);

    // Compose a request
    let client = reqwest::blocking::Client::new();
    let requestbuilder = client.post(&String::from(URL)).multipart(form);

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
