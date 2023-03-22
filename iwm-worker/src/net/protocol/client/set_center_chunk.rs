use crate::{game::player::player_struct::Vec2, net::protocol::utils::buffer_writer::BufferWriter};

#[derive(Debug)]
pub(crate) struct SetCenterChunk {
    x: i32,
    z: i32,
}

impl SetCenterChunk {
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut writer = BufferWriter::new();

        writer.var_int(0x4B);
        writer.var_int(self.x);
        writer.var_int(self.z);

        writer.build()
    }

    pub fn from_tuples((x, z): (i32, i32)) -> SetCenterChunk {
        SetCenterChunk { x, z }
    }
}
