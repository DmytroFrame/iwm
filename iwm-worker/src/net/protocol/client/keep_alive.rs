use crate::net::protocol::utils::buffer_writer::BufferWriter;

#[derive(Debug, PartialEq)]
pub(crate) struct KeepAlive {
    pub keep_alive_id: i64,
}

impl KeepAlive {
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut writer = BufferWriter::new();

        writer.var_int(0x20);
        writer.i64(self.keep_alive_id);

        writer.build()
    }
}
