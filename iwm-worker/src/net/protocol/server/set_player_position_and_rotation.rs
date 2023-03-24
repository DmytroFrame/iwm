use crate::net::protocol::utils::stream_reader::StreamReader;
use tokio::{io::ReadHalf, net::TcpStream};

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
    pub async fn from_stream(stream: &mut ReadHalf<TcpStream>) -> SetPlayerPositionAndRotation {
        let mut reader = StreamReader::new(stream);

        SetPlayerPositionAndRotation {
            x: reader.f64().await,
            y: reader.f64().await,
            z: reader.f64().await,
            yaw: reader.f32().await,
            pitch: reader.f32().await,
            on_ground: reader.bool().await,
        }
    }
}
