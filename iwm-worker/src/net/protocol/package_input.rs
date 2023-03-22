use tokio::{
    io::{AsyncReadExt, ReadHalf},
    net::TcpStream,
};

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
}

pub(crate) async fn input_package_handle(
    size: i32,
    id: i32,
    stream: &mut ReadHalf<TcpStream>,
) -> InputPackage {
    match id {
        0x12 => InputPackage::KeepAlive(KeepAlive::from_stream(stream).await),

        0x14 => InputPackage::SetPlayerPosition(SetPlayerPosition::from_stream(stream).await),

        0x15 => InputPackage::SetPlayerPositionAndRotation(
            SetPlayerPositionAndRotation::from_stream(stream).await,
        ),

        0x16 => InputPackage::SetPlayerRotation(SetPlayerRotation::from_stream(stream).await),

        _ => {
            let mut raw_data = vec![0; size as usize];
            let count = stream.read(&mut raw_data).await.unwrap();

            raw_data.truncate(count);

            let unknown = Unknown { size, id, raw_data };

            // Logger::new("InputPackage:Unknown")
            //     .warn(&format!("Get unknown package: {:?}", unknown));
            InputPackage::Unknown(unknown)
        }
    }
}
