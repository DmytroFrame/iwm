use serde::{Deserialize, Serialize};

// use crate::{game::online, net::{writer::WritePackage, reader::Reader}};

#[derive(Serialize, Deserialize)]
struct StatusResponseVersion {
    pub name: String,
    pub protocol: i32,
}
#[derive(Serialize, Deserialize)]
struct StatusResponsePlayersSample {
    pub name: String,
    pub id: String,
}
#[derive(Serialize, Deserialize)]
struct StatusResponsePlayers {
    pub max: i32,
    pub online: i32,
    pub sample: Vec<StatusResponsePlayersSample>,
}

#[derive(Serialize, Deserialize)]
struct StatusResponseDescription {
    pub text: String,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
struct StatusResponse {
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
                name: "1.12.2".into(),
                protocol: 340,
            },
            players: StatusResponsePlayers {
                max: 2, //online::get_max_online(),
                online: 3, //"online::get_curent_online()",
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




// pub fn pong(value: i64) -> Vec<u8> {
//     let mut package = WritePackage::new();
//     package.byte(0x01);
//     package.long(value);
//     package.build()

// }

// pub fn ping(buf: Vec<u8>) -> i64 {
//     let mut package = Reader::new(&buf);
//     package.byte();
//     package.long()
// }