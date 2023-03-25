use tokio::{
    io::AsyncReadExt,
    net::{TcpListener, TcpStream},
};

use crate::{
    logger::Logger,
    net::{
        handshake_login::handshake_login,
        handshake_status::handshake_status,
        http_status::http_status,
        protocol::server::handshaking::handshake::{Handshake, HandshakeNextState},
    },
};

pub async fn init_server(port: u16) {
    let server = TcpListener::bind(format!("127.0.0.1:{port}"))
        .await
        .unwrap();

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

        0x45 => http_status(stream).await,

        _ => drop(stream),
    }
}

async fn handshaking_handle(mut stream: TcpStream, pkg_size: u8) {
    let mut buffer = vec![0; pkg_size as usize - 1];
    stream.read(&mut buffer).await.unwrap();

    let handshaking = Handshake::from_bytes(&buffer);

    Logger::new("Handshaking").debug(&format!("{:?}", handshaking));

    if handshaking.next_state == HandshakeNextState::Status {
        return handshake_status(stream).await;
    }

    handshake_login(stream).await;
}
