use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::{TcpListener, TcpStream},
};

use crate::{
    logger::Logger,
    net::{
        init_session::init_session,
        protocol::{client::status_response, utils::buffer_writer::BufferWriter},
        protocol::{
            client::status_response::StatusResponse,
            server::handshaking::{Handshaking, HandshakingNextState},
        },
    },
};

pub async fn init_server(_: u16) {
    let server = TcpListener::bind("0.0.0.0:25565").await.unwrap();

    loop {
        let (stream, _) = server.accept().await.unwrap();

        tokio::spawn(async move {
            connection_controller(stream).await;
        });
    }
}

async fn connection_controller(mut stream: TcpStream) {
    let mut buffer = [0; 2];
    stream.read(&mut buffer).await.unwrap();

    let [size, id] = buffer;

    if size == 0 {
        return drop(stream);
    }

    match id {
        0x00 => handshaking_handle(stream, size).await,

        0x45 => http_response(stream).await,

        _ => drop(stream),
    }
}

async fn handshaking_handle(mut stream: TcpStream, pkg_size: u8) {
    let mut buffer = vec![0; pkg_size as usize];
    stream.read(&mut buffer).await.unwrap();

    let handshaking = Handshaking::from_bytes(&buffer);

    Logger::new("Handshaking").debug(&format!("{:?}", handshaking));

    if handshaking.next_state == HandshakingNextState::Status {
        return handshaking_status_response(stream).await;
    }

    init_session(stream).await;
}

async fn handshaking_status_response(mut stream: TcpStream) {
    stream.read(&mut [0; 2]).await.unwrap();

    let mut writer = BufferWriter::new();
    writer.byte(0x00);
    writer.string(StatusResponse::to_string());
    stream.write(&writer.build()).await.unwrap();

    let mut buf: [u8; 10] = [0; 10];
    stream.read(&mut buf).await.unwrap();
    let ping_time = status_response::ping(buf.to_vec());

    stream
        .write(&status_response::pong(ping_time))
        .await
        .unwrap();

    drop(stream);
}

async fn http_response(mut stream: TcpStream) {
    let payload = StatusResponse::to_string();
    let response = format!(
        "HTTP/1.1 200 OK\r\nContent-Length: {}\r\nContent-Type: application/json; charset=utf-8\r\nAccess-Control-Allow-Origin: *\r\nX-Powered-By: iWM Server\r\n\r\n{}",
        payload.len(),
        payload
    );

    stream.write(response.as_bytes()).await.unwrap();

    drop(stream);
}
