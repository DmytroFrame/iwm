use tokio::{io::ReadHalf, net::TcpStream};

use crate::net::protocol::utils::stream_reader::StreamReader;

#[derive(Debug)]
pub(crate) struct SetPlayerPosition {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub on_ground: bool,
}

impl SetPlayerPosition {
    pub async fn from_stream(stream: &mut ReadHalf<TcpStream>) -> SetPlayerPosition {
        let mut reader = StreamReader::new(stream);

        SetPlayerPosition {
            x: reader.f64().await,
            y: reader.f64().await,
            z: reader.f64().await,
            on_ground: reader.bool().await,
        }
    }
}
