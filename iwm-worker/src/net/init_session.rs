use std::time::Duration;

use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::TcpStream,
};

use crate::logger::Logger;

use super::{
    package_queue::create_package_queue,
    protocol::{mock, utils::buffer_writer::BufferWriter},
};

pub async fn init_session(mut stream: TcpStream) {
    // stream.read(&mut [0; 256]).await.unwrap();

    stream
        .write(&[
            0x1e, 0x02, 0xaa, 0x31, 0xd6, 0xfb, 0xe4, 0x69, 0x3a, 0xe4, 0x9b, 0x7f, 0x97, 0xc6,
            0x3c, 0x75, 0x4c, 0x88, 0x0b, 0x44, 0x6d, 0x79, 0x74, 0x72, 0x6f, 0x46, 0x72, 0x61,
            0x6d, 0x65, 0x00,
        ])
        .await
        .unwrap();

    stream.write(mock::LOGIN).await.unwrap();

    let mut writer = BufferWriter::new();
    writer.bytes(&[
        0x39, 0x40, 0x86, 0xEE, 0xA5, 0x6F, 0xF3, 0xA0, 0x86, 0x40, 0x51, 0x74, 0x9A, 0x32, 0x9B,
        0xEE, 0x7D, 0x40, 0x87, 0x9E, 0x79, 0xBF, 0xA5, 0xC9, 0x04, 0x41, 0x51, 0x20, 0xE0, 0x41,
        0x33, 0xFE, 0x9E, 0x00, 0x01, 0x00,
    ]);
    stream.write(&writer.build()).await.unwrap();

    let mut writer = BufferWriter::new();
    writer.bytes(&[0x4B, 0x2D, 0x2F]);
    stream.write(&writer.build()).await.unwrap();

    let mut bufer = vec![0; 4000];

    let count = stream.read(&mut bufer).await.unwrap();

    // buf.truncate(count);
    // let mut buf = &bufer[..=count];

    // println!("len {} {:02X?} ", buf.len(), buf);

    // let mut numb: i32 = 0;
    // loop {
    //     numb += 1 + decode_varint(&mut &buf[(numb as usize)..]).unwrap();
    //     println!("numb {numb}")
    // }

    let (tx, mut rx) = create_package_queue(stream).await;

    while let Some(message) = rx.recv().await {
        Logger::new("InputRX").debug(&format!("{:?}", message));
    }

    tokio::time::sleep(Duration::from_secs(199)).await;
}
fn decode_varint(buf: &mut &[u8]) -> Result<i32, std::io::Error> {
    let mut result = 0;
    let mut shift = 0;

    loop {
        if buf.is_empty() {
            return Err(std::io::Error::new(
                std::io::ErrorKind::UnexpectedEof,
                "Unexpected end of input",
            ));
        }

        let b = buf[0];
        *buf = &buf[1..];

        result |= ((b & 0x7f) as i32) << shift;
        shift += 7;

        if b & 0x80 == 0 {
            break;
        }
    }

    Ok(result)
}
