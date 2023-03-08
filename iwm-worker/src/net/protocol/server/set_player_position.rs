use std::net::TcpStream;

use crate::net::protocol::package_reader::PackageReader;

#[derive(Debug)]
pub(crate) struct SetPlayerPosition {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub on_ground: bool,
}

impl SetPlayerPosition {
    pub fn from(stream: &mut TcpStream) -> SetPlayerPosition {
        let mut reader = PackageReader::new(stream);

        SetPlayerPosition {
            x: reader.f64(),
            y: reader.f64(),
            z: reader.f64(),
            on_ground: reader.bool(),
        }
    }
}