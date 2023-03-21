use tokio::{io::ReadHalf, net::TcpStream};
use crate::net::protocol::utils::stream_reader::StreamReader;

#[derive(Debug)]
pub(crate) struct KeepAlive {
    pub keep_alive_id: i64,
}

impl KeepAlive {
    pub async fn from_stream(stream: &mut ReadHalf<TcpStream>) -> KeepAlive {
        let mut reader = StreamReader::new(stream);

        KeepAlive {
            keep_alive_id: reader.i64().await,
        }
    }
}
