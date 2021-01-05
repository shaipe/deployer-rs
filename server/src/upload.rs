//! copyright © shaipe 2021 - present
//! 服务端文件上传处理
//! create by shaipe 20210104

use actix_multipart::Multipart;
use actix_web::{web, Error as ActixError, HttpRequest, HttpResponse};
use futures::{StreamExt, TryStreamExt};
use std::io::Write;

/// 文件上传处理
pub async fn upload_handler(
    req: HttpRequest,
    mut payload: Multipart,
    // srv: web::Data<Addr<ws::WsServer>>,
) -> Result<HttpResponse, ActixError> {
    println!("{:?}", req);
    // iterate over multipart stream
    while let Ok(Some(mut field)) = payload.try_next().await {
        let content_type = field.content_disposition().unwrap();

        // println!("content type {:?}", field);

        // 获取文件名
        let file_name = match content_type.get_filename() {
            Some(file_name) => file_name,
            None => "",
        };

        // 判断文件名称是否为空
        if file_name.is_empty() {
            continue;
        }

        let filepath = format!("./tmp/{}", sanitize_filename::sanitize(&file_name));

        // File::create is blocking operation, use threadpool
        let mut f = web::block(|| std::fs::File::create(filepath))
            .await
            .unwrap();

        // Field in turn is stream of *Bytes* object
        while let Some(chunk) = field.next().await {
            let data = chunk.unwrap();
            // filesystem operations are blocking, we have to use threadpool
            f = web::block(move || f.write_all(&data).map(|_| f)).await?;
        }
    }
    Ok(HttpResponse::Ok().into())
}
