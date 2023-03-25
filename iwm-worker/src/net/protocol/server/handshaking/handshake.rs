use crate::net::protocol::utils::buffer_reader::BufferReader;

#[derive(Debug, PartialEq)]
pub enum HandshakeNextState {
    Status = 1,
    Login = 2,
}

#[derive(Debug)]
pub(crate) struct Handshake {
    pub protocol_version: i32,
    pub server_host: String,
    pub server_port: u16,
    pub next_state: HandshakeNextState,
}

impl Handshake {
    pub fn from_bytes(buf: &[u8]) -> Handshake {
        let mut reader = BufferReader::new(buf.into());

        Handshake {
            protocol_version: reader.var_int(),
            server_host: reader.string(),
            server_port: reader.u16(),
            next_state: {
                if reader.var_int() == 1 {
                    HandshakeNextState::Status
                } else {
                    HandshakeNextState::Login
                }
            },
        }
    }
}
