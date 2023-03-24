use tokio::{io::ReadHalf, net::TcpStream};

use crate::net::protocol::utils::buffer_reader::BufferReader;

#[derive(Debug)]
pub(crate) struct KeepAlive {
    pub keep_alive_id: i64,
}

impl KeepAlive {
    pub fn from_buffer(buf: Vec<u8>) -> KeepAlive {
        let mut reader = BufferReader::new(buf);

        KeepAlive {
            keep_alive_id: reader.i64(),
        }
    }
}
