use tokio::{io::ReadHalf, net::TcpStream};

use crate::net::protocol::utils::buffer_reader::BufferReader;

#[derive(Debug)]
pub(crate) struct SetPlayerRotation {
    pub x: f32,
    pub y: f32,
    pub on_ground: bool,
}

impl SetPlayerRotation {
    pub fn from_buffer(buf: Vec<u8>) -> SetPlayerRotation {
        let mut reader = BufferReader::new(buf);

        SetPlayerRotation {
            x: reader.f32(),
            y: reader.f32(),
            on_ground: reader.bool(),
        }
    }
}
