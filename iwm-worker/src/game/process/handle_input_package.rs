use crate::{
    game::player::save_player::save_player, logger::Logger,
    net::protocol::package_input::InputPackage,
};

use super::{init_player_session::PlayerSession, init_process::Process};

pub(super) async fn handle_input_package(
    message: InputPackage,
    session: &mut PlayerSession,
    // process: &mut Process,
) {
    match message {
        InputPackage::Disconnect => {
            session.is_disconnected = true;
            save_player(&session.player).await;
        }

        InputPackage::SetPlayerPositionAndRotation(payload) => {
            if (
                payload.x,
                payload.y,
                payload.z,
                payload.yaw,
                payload.pitch,
                payload.on_ground,
            ) != (
                session.player.position.x,
                session.player.position.y,
                session.player.position.z,
                session.player.rotation.x,
                session.player.rotation.z,
                session.player.on_ground,
            ) {
                session.player.position.x = payload.x;
                session.player.position.y = payload.y;
                session.player.position.z = payload.z;
                session.player.rotation.x = payload.yaw;
                session.player.rotation.z = payload.pitch;
                session.player.on_ground = payload.on_ground;

                session.check_chunk_center().await;
            }
        }

        InputPackage::SetPlayerPosition(payload) => {
            if (payload.x, payload.y, payload.z, payload.on_ground)
                != (
                    session.player.position.x,
                    session.player.position.y,
                    session.player.position.z,
                    session.player.on_ground,
                )
            {
                session.player.position.x = payload.x;
                session.player.position.y = payload.y;
                session.player.position.z = payload.z;
                session.player.on_ground = payload.on_ground;

                session.check_chunk_center().await;
            }
        }

        InputPackage::SetPlayerRotation(payload) => {
            if (payload.x, payload.y, payload.on_ground)
                != (
                    session.player.rotation.x,
                    session.player.rotation.z,
                    session.player.on_ground,
                )
            {
                session.player.rotation.x = payload.x;
                session.player.rotation.z = payload.y;
                session.player.on_ground = payload.on_ground;
            }
        }

        InputPackage::ChatMessage(payload) => Logger::new("ChatMessage")
            .info(&format!("{}: {}", session.player.username, payload.message)),

        InputPackage::ChatCommand(payload) => Logger::new("ChatCommand")
            .info(&format!("{}: {}", session.player.username, payload.message)),

        _ => {}
    }
}
