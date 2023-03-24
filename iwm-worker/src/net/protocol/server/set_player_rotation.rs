use crate::net::protocol::utils::stream_reader::StreamReader;
use tokio::{io::ReadHalf, net::TcpStream};

#[derive(Debug)]
pub(crate) struct SetPlayerRotation {
    pub x: f32,
    pub y: f32,
    pub on_ground: bool,
}

impl SetPlayerRotation {
    pub async fn from_stream(stream: &mut ReadHalf<TcpStream>) -> SetPlayerRotation {
        let mut reader = StreamReader::new(stream);

        SetPlayerRotation {
            x: reader.f32().await,
            y: reader.f32().await,
            on_ground: reader.bool().await,
        }
    }
}
