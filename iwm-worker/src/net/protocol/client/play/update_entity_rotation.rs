use crate::{
    game::process::init_player_session::PlayerSession,
    net::protocol::utils::buffer_writer::BufferWriter,
};

#[derive(Debug, PartialEq)]
pub(crate) struct UpdateEntityRotation {
    entity_id: i32,
    yaw: f32,
    pitch: f32,
    on_ground: bool,
}
impl UpdateEntityRotation {
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut writer = BufferWriter::new();

        writer.var_int(0x29);

        writer.var_int(self.entity_id);
        writer.byte(120);
        writer.byte(120);
        writer.bool(self.on_ground);

        writer.build()
    }

    pub fn from_session(sesion: &PlayerSession) -> Self {
        UpdateEntityRotation {
            entity_id: sesion.player.entity_id,
            yaw: sesion.player.rotation.x,
            pitch: sesion.player.rotation.z,
            on_ground: sesion.player.on_ground,
        }
    }
}
