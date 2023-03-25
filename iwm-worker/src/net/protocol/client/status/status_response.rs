use serde::{Deserialize, Serialize};

use crate::{
    game::{
        online,
        shared_constants::{NETWORK_PROTOCOL_VERSION, VERSION_STRING},
    },
    net::protocol::utils::buffer_writer::BufferWriter,
};

#[derive(Serialize, Deserialize)]
pub(crate) struct StatusResponseVersion {
    pub name: String,
    pub protocol: i32,
}
#[derive(Serialize, Deserialize)]
pub(crate) struct StatusResponsePlayers {
    pub max: i32,
    pub online: i32,
    pub sample: Vec<online::SamplePlayer>,
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
    pub async fn to_string() -> String {
        let respons = StatusResponse {
            version: StatusResponseVersion {
                name: VERSION_STRING.to_string(),
                protocol: NETWORK_PROTOCOL_VERSION,
            },
            players: StatusResponsePlayers {
                max: online::get_max_online().await,
                online: online::get_current_online().await,
                sample: online::get_sample_playes().await,
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

    pub async fn to_bytes() -> Vec<u8> {
        let mut writer = BufferWriter::new();

        writer.var_int(0x00);
        writer.string(Self::to_string().await);

        writer.build()
    }
}
