use std::net::TcpStream;

use crate::net::protocol::package_reader::PackageReader;

#[derive(Debug)]
pub(crate) struct SetPlayerRotation {
    pub x: f32,
    pub y: f32,
    pub on_ground: bool,
}

impl SetPlayerRotation {
    pub fn from(stream: &mut TcpStream) -> SetPlayerRotation {
        let mut reader = PackageReader::new(stream);

        SetPlayerRotation {
            x: reader.f32(),
            y: reader.f32(),
            on_ground: reader.bool(),
        }
    }
}