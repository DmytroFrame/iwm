use crate::net::protocol::utils::buffer_reader::BufferReader;

pub(crate) struct PingRequest {
    pub payload: i64,
}

impl PingRequest {
    pub fn from_buffer(buf: Vec<u8>) -> PingRequest {
        let mut reader = BufferReader::new(buf);

        PingRequest {
            payload: reader.i64(),
        }
    }
}
