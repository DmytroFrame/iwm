use uuid::Uuid;

use crate::{
    game::player::player_struct::{Gamemode, Player},
    net::protocol::utils::buffer_writer::BufferWriter,
};

#[derive(Debug, PartialEq)]
pub(crate) struct PlayerInfo {
    action: i32,
    number_of_players: i32,
    uuid: Uuid,
    name: String,
    number_of_properties: i32,
    gamemode: Gamemode,
    ping: i32,
}

impl PlayerInfo {
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut writer = BufferWriter::new();

        writer.var_int(0x37);

        writer.var_int(self.action);
        writer.var_int(1);
        writer.uuid(self.uuid);
        writer.string(self.name.to_string());
        writer.var_int(0);
        writer.var_int(self.gamemode.as_i8() as i32);
        writer.var_int(self.ping);
        writer.byte(0x00);
        writer.byte(0x00);

        writer.build()
    }

    pub fn from_player(player: &Player) -> PlayerInfo {
        PlayerInfo {
            action: 0,
            number_of_players: 1,
            uuid: player.uuid,
            name: player.username.to_string(),
            number_of_properties: 0,
            gamemode: player.gamemode,
            ping: 1,
        }
    }
}
