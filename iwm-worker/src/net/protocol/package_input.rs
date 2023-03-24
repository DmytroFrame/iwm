use crate::logger::Logger;

use super::server::{
    keep_alive::KeepAlive, set_player_position::SetPlayerPosition,
    set_player_position_and_rotation::SetPlayerPositionAndRotation,
    set_player_rotation::SetPlayerRotation, unknown::Unknown,
};

#[derive(Debug)]
pub(crate) enum InputPackage {
    KeepAlive(KeepAlive),
    SetPlayerPosition(SetPlayerPosition),
    SetPlayerPositionAndRotation(SetPlayerPositionAndRotation),
    SetPlayerRotation(SetPlayerRotation),
    Unknown(Unknown),
    Disconnect,
}

pub(crate) fn input_package_handle(id: i32, buffer: Vec<u8>) -> InputPackage {
    match id {
        0x12 => InputPackage::KeepAlive(KeepAlive::from_buffer(buffer)),

        0x14 => InputPackage::SetPlayerPosition(SetPlayerPosition::from_buffer(buffer)),

        0x15 => InputPackage::SetPlayerPositionAndRotation(
            SetPlayerPositionAndRotation::from_buffer(buffer),
        ),

        0x16 => InputPackage::SetPlayerRotation(SetPlayerRotation::from_buffer(buffer)),

        _ => {
            let unknown = Unknown {
                id,
                size: buffer.len() as i32 + 1,
                raw_data: buffer,
            };

            Logger::new("ReaderIO").error(&format!(
                "size: {} id: 0x{:X}, data: {:02X?}",
                unknown.size, unknown.id, unknown.raw_data
            ));

            InputPackage::Unknown(unknown)
        }
    }
}
