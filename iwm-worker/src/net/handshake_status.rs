use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::TcpStream,
};

use super::protocol::{
    client::status::{ping_response::PingResponse, status_response::StatusResponse},
    server::status::ping_request::PingRequest,
};

pub async fn handshake_status(mut stream: TcpStream) {
    stream.read(&mut [0; 2]).await.unwrap();

    stream
        .write(&StatusResponse::to_bytes().await)
        .await
        .unwrap();

    let mut buf: [u8; 10] = [0; 10];
    stream.read(&mut buf).await.unwrap();
    let ping = PingRequest::from_buffer(buf.to_vec());

    let ping_response = PingResponse {
        payload: ping.payload,
    };

    stream.write(&ping_response.to_bytes()).await.unwrap();

    drop(stream);
}
