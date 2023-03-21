use serde::{Deserialize, Serialize};

use crate::net::protocol::utils::{buffer_reader::BufferReader, buffer_writer::BufferWriter};

// use crate::{game::online, net::{writer::WritePackage, reader::Reader}};

#[derive(Serialize, Deserialize)]
pub(crate) struct StatusResponseVersion {
    pub name: String,
    pub protocol: i32,
}
#[derive(Serialize, Deserialize)]
pub(crate) struct StatusResponsePlayersSample {
    pub name: String,
    pub id: String,
}
#[derive(Serialize, Deserialize)]
pub(crate) struct StatusResponsePlayers {
    pub max: i32,
    pub online: i32,
    pub sample: Vec<StatusResponsePlayersSample>,
}

#[derive(Serialize, Deserialize)]
pub(crate) struct StatusResponseDescription {
    pub text: String,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub(crate) struct StatusResponse {
    pub version: StatusResponseVersion,
    pub players: StatusResponsePlayers,
    pub description: StatusResponseDescription,
    pub favicon: String,
    pub previewsChat: bool,
    pub enforcesSecureChat: bool,
}

impl StatusResponse {
    pub fn to_string() -> String {
        let respons = StatusResponse {
            version: StatusResponseVersion {
                name: "1.19.2".into(),
                protocol: 760,
            },
            players: StatusResponsePlayers {
                max: 20,   //online::get_max_online(),
                online: 0, //"online::get_curent_online()",
                sample: Vec::new(),
            },
            description: StatusResponseDescription {
                text: "This minecraft server.".into(),
            },
            favicon: "".into(),
            enforcesSecureChat: true,
            previewsChat: true,
        };

        serde_json::to_string(&respons).unwrap()
    }
}

pub fn pong(value: i64) -> Vec<u8> {
    let mut package = BufferWriter::new();
    package.byte(0x01);
    package.i64(value);
    package.build()
}

pub fn ping(buf: Vec<u8>) -> i64 {
    let mut package = BufferReader::new(&buf);
    package.byte();
    package.i64()
}
