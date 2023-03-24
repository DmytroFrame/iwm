use crate::net::protocol::utils::buffer_writer::BufferWriter;

#[derive(Debug, PartialEq)]
pub(crate) struct ChunkDataAndUpdateLight {
    pub x: i32,
    pub z: i32,
    pub raw_data: Vec<u8>,
}

impl ChunkDataAndUpdateLight {
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut writer = BufferWriter::new();

        writer.byte(0x21);
        writer.i32(self.x);
        writer.i32(self.z);
        writer.bytes(&self.raw_data);

        writer.build()
    }
}
