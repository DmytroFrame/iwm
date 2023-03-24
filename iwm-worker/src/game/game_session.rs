use std::time::Duration;

use tokio::time::Instant;
use uuid::Uuid;

use crate::{
    logger::Logger,
    net::{
        package_queue::PlayerStream,
        protocol::client::keep_alive::KeepAlive,
        protocol::package_output::OutputPackage,
        protocol::{client::set_center_chunk::SetCenterChunk, package_input::InputPackage},
    },
};

use super::{
    chunk::{get_chunk_radius::get_chunk_radius, get_flat_chunk::get_flat_chunk},
    player::player_struct::{Gamemode, Player, Vec2, Vec3},
};

struct Session {
    player: Player,
    stream: PlayerStream,
    chunk_center: Vec2<i32>,
}

impl Session {
    pub fn new(stream: PlayerStream) -> Session {
        Session {
            stream,
            chunk_center: Vec2 { x: 2, z: 0 },
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
        // println!("{:?}", get_chunk_radius(current_x, current_z, radius));

        // for vec2 in get_chunk_radius(current_x, current_z, radius) {
        //     // println!("{:?}", vec2);

        //     self.stream
        //         .output
        //         .send(OutputPackage::ChunkDataAndUpdateLight(get_flat_chunk(
        //             vec2.x, vec2.z,
        //         )))
        //         .await
        //         .unwrap();
        //     // tokio::time::sleep(Duration::from_millis(100)).await;
        // }
    }
}

struct Process {
    players: Vec<Session>,
    entities: Vec<String>,
    chunks: Vec<String>,
}

pub(crate) async fn game_session(stream: PlayerStream) {
    let log = Logger::new("GameSession");

    let mut session = Session::new(stream);
    let mut last_keep_alive = Instant::now();

    // session.send_chunk().await;

    while let Some(message) = session.stream.input.recv().await {
        if Instant::now().duration_since(last_keep_alive) > Duration::from_secs(20) {
            last_keep_alive = Instant::now();

            let package = OutputPackage::KeepAlive(KeepAlive {
                keep_alive_id: 1337,
            });
            session.stream.output.send(package).await.unwrap();
        }

        match message {
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
            any => {}
        }
    }
}
