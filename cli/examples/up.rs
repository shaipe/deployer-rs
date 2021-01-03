// extern crate multipart;
extern crate reqwest;
// use multipart::client::lazy::Multipart;
// use reqwest::mime;
use std::io::Read;
use std::process;

const URL: &'static str = "http://localhost:3000";
const UPLOAD_FORM_FIELD_NAME: &'static str = "thefile";
const FILENAME: &'static str = "sample.txt";

/// This is example code for using the multipart crate with reqwest to
/// upload a file.  The multipart crate is used to encode the file, and
/// the necessary headers/body are manually added to the reqwest
/// request.
///
/// NOTE: Built-in multipart support in reqwest is pending, and this
/// example will be obsolete when that is available.
///
/// NOTE: For simplicity, this example stores the entire encoded file in
/// memory.  For anything but very small files, you'd want a streaming
/// approach instead.
fn main() {
    use reqwest::blocking::multipart;

    let form = multipart::Form::new()
        // Adding just a simple text field...
        // .text("username", "seanmonstar")
        // And a file...
        .file("photo", "photo.png").unwrap();

    // Customize all the details of a Part if needed...
    let bio = multipart::Part::text("hallo peeps")
        .file_name("sample.txt")
        .mime_str("text/plain").unwrap();

    // Add the custom part to our form...
    let form = form.part("biography", bio);

    // Construct a multipart description
    // let mut multipart = Multipart::new();
    // multipart.add_file(UPLOAD_FORM_FIELD_NAME, FILENAME);
    // let mut multipart_prepared = multipart.prepare().unwrap();
    // let mut multipart_buffer: Vec<u8> = vec![];
    // multipart_prepared
    //     .read_to_end(&mut multipart_buffer)
    //     .unwrap();

    // // let reader = std::fs::File::open("test.txt").unwrap();
    // let part = reqwest::blocking::multipart::Part::bytes(multipart_buffer);
    // let form = reqwest::blocking::multipart::Form::new();
    // let form = form.part("test", part);
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
}
