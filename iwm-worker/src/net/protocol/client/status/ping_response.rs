use crate::net::protocol::utils::buffer_writer::BufferWriter;

pub(crate) struct PingResponse {
    pub payload: i64,
}

impl PingResponse {
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut writer = BufferWriter::new();

        writer.var_int(0x01);
        writer.i64(self.payload);

        writer.build()
    }
}
