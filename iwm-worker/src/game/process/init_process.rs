use tokio::time::Instant;

use crate::{
    game::{chunk::get_flat_chunk::get_flat_chunk, online, player::player_struct::Player},
    logger::Logger,
    net::{
        package_queue::PlayerStream,
        protocol::{
            client::play::{
                set_center_chunk::SetCenterChunk, set_render_distance::SetRenderDistance,
                synchronize_player_position::SynchronizePlayerPosition,
            },
            package_output::OutputPackage,
        },
    },
    utils::vec2::Vec2,
};

use super::game_process::game_process;

pub(super) struct PlayerSession {
    pub player: Player,
    pub stream: PlayerStream,
    pub last_keep_alive: Instant,
    pub chunk_center: Vec2<i32>,
    pub is_disconnected: bool,
}

impl PlayerSession {
    pub fn new(stream: PlayerStream, player: Player) -> PlayerSession {
        PlayerSession {
            stream,
            chunk_center: Vec2 { x: 2, z: 0 },
            last_keep_alive: Instant::now(),
            is_disconnected: false,
            player,
        }
    }

    pub async fn check_chunk_center(&mut self) {
        let new_position = self.player.get_chunk_position();

        if (self.chunk_center.x, self.chunk_center.z) != new_position {
            (self.chunk_center.x, self.chunk_center.z) = new_position;
            self.stream
                .output
                .send(OutputPackage::SetCenterChunk(SetCenterChunk::from_tuples(
                    new_position,
                )))
                .await
                .unwrap();

            self.send_chunk().await;
        }
    }

    pub async fn send_chunk(&mut self) {
        // let radius = 8;
        let current_x = self.chunk_center.x.clone();
        let current_z = self.chunk_center.z.clone();

        self.stream
            .output
            .send(OutputPackage::ChunkDataAndUpdateLight(get_flat_chunk(
                current_x, current_z,
            )))
            .await
            .unwrap();
    }
}

pub(super) struct Process {
    pub players: Vec<PlayerSession>,
    pub events: Vec<String>,
    pub chunks: Vec<String>,
}

impl Process {
    pub fn is_all_disconnected(&mut self) -> bool {
        for player in &mut self.players {
            if player.is_disconnected == false {
                return false;
            }
        }
        return true;
    }
}

pub(crate) async fn init_process(stream: PlayerStream, player: Player) {
    let player_session = PlayerSession::new(stream, player);

    let mut process = Process {
        players: vec![player_session],
        events: vec![],
        chunks: vec![],
    };

    process.players[0]
        .stream
        .output
        .send(OutputPackage::SetRenderDistance(SetRenderDistance {
            view_distance: 32,
        }))
        .await
        .unwrap();

    process.players[0]
        .stream
        .output
        .send(OutputPackage::SynchronizePlayerPosition(
            SynchronizePlayerPosition::from_player(&process.players[0].player),
        ))
        .await
        .unwrap();

    loop {
        game_process(&mut process).await;

        if process.is_all_disconnected() {
            Logger::new("Process").info("Process is ended");
            online::minus_online(&process.players[0].player.username).await;
            break;
        }
    }
}
