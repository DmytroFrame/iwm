use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::TcpStream,
};

use super::protocol::client::status::status_response::StatusResponse;

pub async fn http_status(mut stream: TcpStream) {
    stream.read(&mut [0; 2048]).await.unwrap();

    let payload = StatusResponse::to_string(None).await;
    let response = format!(
        "HTTP/1.1 200 OK\r\nContent-Length: {}\r\nContent-Type: application/json; charset=utf-8\r\nAccess-Control-Allow-Origin: *\r\nX-Powered-By: iWM Server\r\n\r\n{}",
        payload.len(),
        payload
    );

    stream.write(response.as_bytes()).await.unwrap();

    drop(stream);
}
