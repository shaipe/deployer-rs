//! copyright © shaipe 2021 - present
//! 本地文件上传到服务器
//! create by shaipe 20210103

pub fn upload_file() -> Result<(), reqwest::Error> {
    use reqwest::multipart;

    let form = multipart::Form::new()
        // Adding just a simple text field...
        // .text("username", "seanmonstar")
        // And a file...
        .file("photo", "photo.png")?;

    // Customize all the details of a Part if needed...
    let bio = multipart::Part::text("hallo peeps")
        .file_name("bio.txt")
        .mime_str("text/plain")?;

    // Add the custom part to our form...
    let form = form.part("biography", bio);

    // And finally, send the form
    let client = reqwest::Client::new();
    let resp = client
        .post("http://localhost:3000/")
        .multipart(form)
        .send()?;
}
