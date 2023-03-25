use crate::net::protocol::utils::buffer_writer::BufferWriter;

#[derive(Debug, PartialEq)]
pub(crate) struct SetRenderDistance {
    pub view_distance: i32,
}

impl SetRenderDistance {
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut writer = BufferWriter::new();

        writer.var_int(0x4C);
        writer.var_int(self.view_distance);

        writer.build()
    }
}
