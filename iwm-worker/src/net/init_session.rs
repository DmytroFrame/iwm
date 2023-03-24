use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::TcpStream,
};

use crate::{
    game::process::init_process::init_process, net::protocol::server::login_start::LoginStart,
};

use super::{
    package_queue::create_package_queue,
    protocol::{
        mock,
        utils::{buffer_writer::BufferWriter, tcp_stream_reader::TcpStreamReader},
    },
};

pub async fn init_session(mut stream: TcpStream) {
    let mut reader = TcpStreamReader::new(&mut stream);

    let size = reader.var_int().await;
    let _id = reader.var_int().await;

    let mut buf = vec![0; size as usize - 1];
    stream.read(&mut buf).await.unwrap();
    println!("{:?}", LoginStart::from_buffer(buf));

    stream
        .write(&[
            0x1e, 0x02, 0xaa, 0x31, 0xd6, 0xfb, 0xe4, 0x69, 0x3a, 0xe4, 0x9b, 0x7f, 0x97, 0xc6,
            0x3c, 0x75, 0x4c, 0x88, 0x0b, 0x44, 0x6d, 0x79, 0x74, 0x72, 0x6f, 0x46, 0x72, 0x61,
            0x6d, 0x65, 0x00,
        ])
        .await
        .unwrap();

    stream.write(mock::LOGIN).await.unwrap();


    let player_stream = create_package_queue(stream).await;

    init_process(player_stream).await;
}
