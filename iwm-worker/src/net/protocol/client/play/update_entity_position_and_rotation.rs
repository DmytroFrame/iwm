use crate::{
    game::process::init_player_session::PlayerSession,
    net::protocol::utils::buffer_writer::BufferWriter, utils::delta::get_delta_position,
};

#[derive(Debug, PartialEq)]
pub(crate) struct UpdateEntityPositionAndRotation {
    entity_id: i32,
    delta_x: i16,
    delta_y: i16,
    delta_z: i16,
    yaw: f32,
    pitch: f32,
    on_ground: bool,
}

impl UpdateEntityPositionAndRotation {
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut writer = BufferWriter::new();

        writer.var_int(0x29);

        writer.var_int(self.entity_id);
        writer.i16(self.delta_x);
        writer.i16(self.delta_y);
        writer.i16(self.delta_z);
        writer.byte(self.yaw as u8);
        writer.byte(self.pitch as u8);
        writer.bool(self.on_ground);

        writer.build()
    }

    pub fn from_session(sesion: &PlayerSession) -> Self {
        UpdateEntityPositionAndRotation {
            entity_id: sesion.player.entity_id,
            delta_x: get_delta_position(sesion.player.position.x, sesion.previous_position.x),
            delta_y: get_delta_position(sesion.player.position.y, sesion.previous_position.y),
            delta_z: get_delta_position(sesion.player.position.z, sesion.previous_position.z),
            yaw: sesion.player.rotation.x,
            pitch: sesion.player.rotation.z,
            on_ground: sesion.player.on_ground,
        }
    }
}
