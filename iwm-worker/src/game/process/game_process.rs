use std::time::Duration;

use tokio::time::Instant;

use crate::net::protocol::{
    client::play::{keep_alive::KeepAlive, player_info::PlayerInfo, spawn_player::SpawnPlayer},
    package_output::OutputPackage,
};

use super::{handle_input_package::handle_input_package, init_process::Process};

pub(super) async fn game_process(process: &mut Process) {
    let mut last_eid: i32 = 0;

    for session in &mut process.players {
        last_eid = session.player.entity_id;

        loop {
            match session.stream.input.try_recv() {
                Err(_) => break,

                Ok(message) => {
                    // println!("message: {:?}", message);
                    handle_input_package(message, session).await;
                }
            }
        }

        if Instant::now().duration_since(session.last_keep_alive) > Duration::from_secs(20) {
            session.last_keep_alive = Instant::now();

            let package = OutputPackage::KeepAlive(KeepAlive {
                keep_alive_id: 1337,
            });
            session.stream.output.send(package).await.unwrap();
        }
    }

    if last_eid != process.players[0].player.entity_id {
        process.players[0]
            .stream
            .output
            .send(OutputPackage::PlayerInfo(PlayerInfo::from_player(
                &process.players[1].player,
            )))
            .await
            .unwrap();

        // println!("{:02X?}", PlayerInfo::from_player(&process.players[1].player).to_bytes());

        process.players[0]
            .stream
            .output
            .send(OutputPackage::SpawnPlayer(SpawnPlayer::from_player(
                &process.players[1].player,
            )))
            .await
            .unwrap();
    }
}
