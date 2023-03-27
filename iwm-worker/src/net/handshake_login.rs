use crate::{
    game::{
        online, player::get_player::get_player, process::init_player_session::init_player_session,
    },
    net::protocol::{
        client::login::login_success::LoginSuccess, server::login::login_start::LoginStart,
        utils::package_header::PackageHeader,
    },
};
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::TcpStream,
};

use super::{package_queue::create_package_queue, protocol::client::play::login::LoginPlay};

pub async fn handshake_login(mut stream: TcpStream) {
    let header = PackageHeader::from_steam(&mut stream).await.unwrap();
    let mut buf = vec![0; header.size as usize - 1];
    stream.read(&mut buf).await.unwrap();

    let login_start = LoginStart::from_buffer(buf);

    let player = get_player(&login_start.username).await;

    let login = LoginSuccess {
        username: player.username.to_string(),
        uuid: player.uuid,
    };
    stream.write(&login.to_bytes()).await.unwrap();
    stream.write(&LoginPlay::new().to_bytes()).await.unwrap();

    let player_stream = create_package_queue(stream).await;
    online::add_online(&player.username, &player.uuid).await;

    init_player_session(player_stream, player).await;
}
