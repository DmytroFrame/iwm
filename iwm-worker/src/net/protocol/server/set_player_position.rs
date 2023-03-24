use tokio::{io::ReadHalf, net::TcpStream};

use crate::net::protocol::utils::buffer_reader::BufferReader;

#[derive(Debug)]
pub(crate) struct SetPlayerPosition {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub on_ground: bool,
}

impl SetPlayerPosition {
    pub fn from_buffer(buf: Vec<u8>) -> SetPlayerPosition {
        let mut reader = BufferReader::new(buf);

        SetPlayerPosition {
            x: reader.f64(),
            y: reader.f64(),
            z: reader.f64(),
            on_ground: reader.bool(),
        }
    }
}
