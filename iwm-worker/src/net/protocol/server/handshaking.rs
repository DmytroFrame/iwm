use crate::net::protocol::utils::{buffer_reader::BufferReader, stream_reader::StreamReader};
use tokio::{io::ReadHalf, net::TcpStream};

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
    // pub async fn from_stream(stream: &mut ReadHalf<TcpStream>) -> Handshaking {
    //     let mut reader = StreamReader::new(stream);

    //     Handshaking {
    //         protocol_version: reader.var_int().await,
    //         server_host: reader.string().await,
    //         server_port: reader.u16().await,
    //         next_state: {
    //             if reader.var_int().await == 1 {
    //                 HandshakingNextState::Status
    //             } else {
    //                 HandshakingNextState::Login
    //             }
    //         },
    //     }
    // }

    pub fn from_bytes(buf: &[u8]) -> Handshaking {
        let mut reader = BufferReader::new(buf);

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
