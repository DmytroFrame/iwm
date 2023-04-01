use crate::{
    game::{player::player_struct::Player, process::init_player_session::PlayerSession},
    net::protocol::utils::buffer_writer::BufferWriter,
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
        let x = ((sesion.player.position.x.floor() as i64 * 32
            - sesion.previous_position.x.floor() as i64 * 32)
            * 128) as i16;

        let y = ((sesion.player.position.y.floor() as i64 * 32
            - sesion.previous_position.y.floor() as i64 * 32)
            * 128) as i16;

        let z = ((sesion.player.position.z.floor() as i64 * 32
            - sesion.previous_position.z.floor() as i64 * 32)
            * 128) as i16;

        // println!("{} {} {}", x / (128 * 32), y / (128 * 32), z / (128 * 32));

        UpdateEntityPosition {
            entity_id: sesion.player.entity_id,
            delta_x: x,
            delta_y: y,
            delta_z: z,
            on_ground: sesion.player.on_ground,
        }
    }
}
