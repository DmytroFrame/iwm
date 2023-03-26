use crate::{
    game::player::player_struct::Player, net::protocol::utils::buffer_writer::BufferWriter,
};

#[derive(Debug, PartialEq)]
pub(crate) struct SynchronizePlayerPosition {
    x: f64,
    y: f64,
    z: f64,
    yaw: f32,
    pitch: f32,
    flags: i8,
    teleport_id: i32,
    dismount_vehicle: bool,
}

impl SynchronizePlayerPosition {
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut writer = BufferWriter::new();

        writer.var_int(0x39);

        writer.f64(self.x);
        writer.f64(self.y);
        writer.f64(self.z);
        writer.f32(self.yaw);
        writer.f32(self.pitch);
        writer.i8(self.flags);
        writer.var_int(self.teleport_id);
        writer.bool(self.dismount_vehicle);

        writer.build()
    }

    pub fn from_player(player: &Player) -> SynchronizePlayerPosition {
        SynchronizePlayerPosition {
            x: player.position.x,
            y: player.position.y,
            z: player.position.z,
            yaw: player.rotation.x,
            pitch: player.rotation.z,
            flags: 0,
            teleport_id: 1,
            dismount_vehicle: false,
        }
    }
}
