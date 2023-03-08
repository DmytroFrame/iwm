use std::net::TcpStream;

use crate::net::protocol::package_reader::PackageReader;

#[derive(Debug, PartialEq)]
pub enum HandshakingNextState {
    Status = 1,
    Login = 2,
}

#[derive(Debug)]
pub(crate) struct Handshaking {
    pub protocol_version: i32,
    pub server_host: String,
    pub server_port: u16,
    pub next_state: HandshakingNextState,
}

impl Handshaking {
    pub fn from(stream: &mut TcpStream) -> Handshaking {
        let mut reader = PackageReader::new(stream);

        Handshaking {
            protocol_version: reader.var_int(),
            server_host: reader.string(),
            server_port: reader.u16(),
            next_state: {
                if reader.var_int() == 1 {
                    HandshakingNextState::Status
                } else {
                    HandshakingNextState::Login
                }
            },
        }
    }
}
