use std::net::TcpStream;

use crate::net::protocol::package_reader::PackageReader;

#[derive(Debug)]
pub(crate) struct SetPlayerPositionAndRotation {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub yaw: f32,
    pub pitch: f32,
    pub on_ground: bool,
}

impl SetPlayerPositionAndRotation {
    pub fn from(stream: &mut TcpStream) -> SetPlayerPositionAndRotation {
        let mut reader = PackageReader::new(stream);

        SetPlayerPositionAndRotation {
            x: reader.f64(),
            y: reader.f64(),
            z: reader.f64(),
            yaw: reader.f32(),
            pitch: reader.f32(),
            on_ground: reader.bool(),
        }
    }
}
