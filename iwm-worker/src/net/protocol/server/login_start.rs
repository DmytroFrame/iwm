use crate::net::protocol::utils::buffer_reader::BufferReader;

#[derive(Debug)]
pub(crate) struct LoginStart {
    pub name: String,
    pub has_sig_data: bool,
    pub timestamp: Option<i64>,
    pub public_key_length: Option<i32>,
    pub public_key: Option<Vec<u8>>,
    pub signature_length: Option<i32>,
    pub signature: Option<Vec<u8>>,
    pub has_player_uuid: bool,
    pub player_uuid: Option<String>,
}

impl LoginStart {
    pub fn from_bytes(buf: Vec<u8>) -> LoginStart {
        let mut reader = BufferReader::new(buf);

        let name = reader.string();
        let has_sig_data = reader.bool();
        let (
            mut timestamp,
            mut public_key_length,
            mut public_key,
            mut signature_length,
            mut signature,
        ) = (None, None, None, None, None);

        if has_sig_data {
            timestamp = Some(reader.i64());
            public_key_length = Some(reader.var_int());
            public_key = Some(reader.bytes(public_key_length.unwrap() as usize));
            signature_length = Some(reader.var_int());
            signature = Some(reader.bytes(signature_length.unwrap() as usize));
        }

        let has_player_uuid = reader.bool();
        let mut player_uuid = None;

        if has_player_uuid {
            player_uuid = Some(reader.uuid());
        }

        LoginStart {
            name,
            has_sig_data,
            timestamp,
            public_key_length,
            public_key,
            signature_length,
            signature,
            has_player_uuid,
            player_uuid,
        }
    }
}
