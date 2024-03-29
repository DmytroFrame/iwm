use crate::{
    game::process::init_player_session::PlayerSession,
    net::protocol::utils::buffer_writer::BufferWriter, utils::delta::get_delta_position,
};

#[derive(Debug, PartialEq)]
pub(crate) struct UpdateEntityPosition {
    entity_id: i32,
    delta_x: i16,
    delta_y: i16,
    delta_z: i16,
    on_ground: bool,
}

impl UpdateEntityPosition {
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut writer = BufferWriter::new();

        writer.var_int(0x28);

        writer.var_int(self.entity_id);
        writer.i16(self.delta_x);
        writer.i16(self.delta_y);
        writer.i16(self.delta_z);
        writer.bool(self.on_ground);
        writer.build()
    }

    pub fn from_session(sesion: &PlayerSession) -> UpdateEntityPosition {
        UpdateEntityPosition {
            entity_id: sesion.player.entity_id,
            delta_x: get_delta_position(sesion.player.position.x, sesion.previous_position.x),
            delta_y: get_delta_position(sesion.player.position.y, sesion.previous_position.y),
            delta_z: get_delta_position(sesion.player.position.z, sesion.previous_position.z),
            on_ground: sesion.player.on_ground,
        }
    }
}
