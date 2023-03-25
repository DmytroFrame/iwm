use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::TcpStream,
};
use uuid::Uuid;

use crate::{
    game::process::init_process::init_process,
    net::protocol::{
        client::login::login_success::LoginSuccess, server::login::login_start::LoginStart,
        utils::package_header::PackageHeader,
    },
};

use super::{package_queue::create_package_queue, protocol::client::play::login::LoginPlay};

pub async fn handshake_login(mut stream: TcpStream) {
    let header = PackageHeader::from_steam(&mut stream).await.unwrap();
    let mut buf = vec![0; header.size as usize - 1];
    stream.read(&mut buf).await.unwrap();

    let login_start = LoginStart::from_buffer(buf);

    // println!("{:?}",);

    let login = LoginSuccess {
        username: "xyucoc".to_string(),
        uuid: Uuid::new_v4(),
    };
    stream.write(&login.to_bytes()).await.unwrap();
    println!("{:?}", LoginPlay::new().to_bytes());
    stream.write(&LoginPlay::new().to_bytes()).await.unwrap();

    let player_stream = create_package_queue(stream).await;

    init_process(player_stream).await;
}
