use tokio::time::Instant;
use uuid::Uuid;

use crate::{
    game::{
        chunk::get_flat_chunk::get_flat_chunk,
        player::player_struct::{Gamemode, Player, Vec2, Vec3},
    },
    net::{
        package_queue::PlayerStream,
        protocol::{client::set_center_chunk::SetCenterChunk, package_output::OutputPackage},
    },
};

use super::game_process::game_process;

pub(super) struct PlayerSession {
    pub player: Player,
    pub stream: PlayerStream,
    pub last_keep_alive: Instant,
    pub chunk_center: Vec2<i32>,
}

impl PlayerSession {
    pub fn new(stream: PlayerStream) -> PlayerSession {
        PlayerSession {
            stream,
            chunk_center: Vec2 { x: 2, z: 0 },
            last_keep_alive: Instant::now(),
            player: Player {
                entity_id: 1,
                username: String::new(),
                uuid: Uuid::new_v4(),
                gamemode: Gamemode::Survival,
                position: Vec3 {
                    x: 0.0,
                    y: 0.0,
                    z: 0.0,
                },
                rotation: Vec2 { x: 0.0, z: 0.0 },
                on_ground: true,
                health: 20.0,
            },
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

pub(crate) async fn init_process(stream: PlayerStream) {
    let player_session = PlayerSession::new(stream);

    let mut process = Process {
        players: vec![player_session],
        events: vec![],
        chunks: vec![],
    };

    loop {
        game_process(&mut process).await;
    }
}