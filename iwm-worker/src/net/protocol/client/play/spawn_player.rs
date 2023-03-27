use uuid::Uuid;

use crate::{
    game::player::player_struct::Player, net::protocol::utils::buffer_writer::BufferWriter,
};

#[derive(Debug, PartialEq)]
pub(crate) struct SpawnPlayer {
    entity_id: i32,
    uuid: Uuid,
    x: f64,
    y: f64,
    z: f64,
    yaw: u8,
    pitch: u8,
}

impl SpawnPlayer {
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut writer = BufferWriter::new();

        writer.var_int(0x02);

        writer.var_int(self.entity_id);
        writer.uuid(self.uuid);
        writer.f64(self.x);
        writer.f64(self.y);
        writer.f64(self.z);
        writer.u8(self.yaw);
        writer.u8(self.pitch);

        writer.build()
    }

    pub fn from_player(player: &Player) -> SpawnPlayer {
        SpawnPlayer {
            entity_id: player.entity_id,
            uuid: player.uuid,
            x: player.position.x,
            y: player.position.y,
            z: player.position.z,
            yaw: player.rotation.x.floor() as u8,
            pitch: player.rotation.z.floor() as u8,
        }
    }
}
