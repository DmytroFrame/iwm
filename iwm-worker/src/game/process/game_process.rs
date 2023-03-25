use std::time::Duration;

use tokio::time::Instant;

use crate::{
    logger::Logger,
    net::protocol::{
        client::play::keep_alive::KeepAlive, package_input::InputPackage,
        package_output::OutputPackage,
    },
};

use super::init_process::Process;

pub(super) async fn game_process(process: &mut Process) {
    for session in &mut process.players {
        while let Some(message) = session.stream.input.recv().await {
            match message {
                InputPackage::Disconnect => {
                    session.is_disconnected = true;
                    break;
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

            if Instant::now().duration_since(session.last_keep_alive) > Duration::from_secs(20) {
                session.last_keep_alive = Instant::now();

                let package = OutputPackage::KeepAlive(KeepAlive {
                    keep_alive_id: 1337,
                });
                session.stream.output.send(package).await.unwrap();
            }
        }
    }
}
