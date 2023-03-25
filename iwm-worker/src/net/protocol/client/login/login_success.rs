use uuid::Uuid;

use crate::net::protocol::utils::buffer_writer::BufferWriter;

pub(crate) struct LoginSuccess {
    pub username: String,
    pub uuid: Uuid,
}

impl LoginSuccess {
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut writer = BufferWriter::new();

        writer.var_int(0x02);
        writer.uuid(self.uuid);
        writer.string(self.username.clone());

        // Number Of Properties
        writer.var_int(0);

        writer.build()
    }
}
