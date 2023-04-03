use std::time::Duration;

use tokio::time::Instant;

use crate::{
    game::{
        chunk::{
            chunk_owner::{get_chunk_owner, set_chunk_owner},
            get_chunk_tracer::get_chunk_tracer,
            get_flat_chunk::get_flat_chunk,
        },
        event::manager::EventMenager,
        player::{
            events::{
                join_player::join_player_handler,
                update_entity_position::update_entity_position_handler,
                update_entity_position_and_rotation::update_entity_position_and_rotation_handler,
                update_entity_rotation::update_entity_rotation_handler,
            },
            player_struct::Player,
        },
    },
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
    utils::{vec2::Vec2, vec3::Vec3},
};

use super::{init_process::init_process, process_channels::send_session_to_process};

#[derive(Debug)]
pub(crate) struct PlayerSession {
    pub player: Player,
    pub previous_position: Vec3<f64>,
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
            previous_position: Vec3 {
                x: player.position.x,
                y: player.position.y,
                z: player.position.z,
            },
            player,
        }
    }

    pub async fn check_chunk_center(&mut self) {
        let new_position = self.player.get_chunk_position();

        if (self.chunk_center.x, self.chunk_center.z) != new_position {
            (self.chunk_center.x, self.chunk_center.z) = new_position;

            // println!("new_position {:?} ", new_position);
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

        for Vec2 { x, z } in get_chunk_tracer(current_x, current_z, 3) {
            tokio::time::sleep(Duration::from_millis(20)).await;
            set_chunk_owner((x, z), self.player.entity_id).await;

            self.stream
                .output
                .send(OutputPackage::ChunkDataAndUpdateLight(get_flat_chunk(x, z)))
                .await
                .unwrap();
        }
    }

    pub fn init_event_handlers(&mut self, event: &mut EventMenager) {
        println!("init player Handler");

        let id = self.player.entity_id;

        event.add_handler(
            crate::game::event::events::Events::PlayerJoin,
            id,
            join_player_handler,
        );

        event.add_handler(
            crate::game::event::events::Events::UpdateEntityPosition,
            id,
            update_entity_position_handler,
        );

        event.add_handler(
            crate::game::event::events::Events::UpdateEntityPositionAndRotation,
            id,
            update_entity_position_and_rotation_handler,
        );

        event.add_handler(
            crate::game::event::events::Events::UpdateEntityRotation,
            id,
            update_entity_rotation_handler,
        )
    }
}

pub(crate) async fn init_player_session(stream: PlayerStream, player: Player) {
    let session = PlayerSession::new(stream, player);

    session
        .stream
        .output
        .send(OutputPackage::SetRenderDistance(SetRenderDistance {
            view_distance: 32,
        }))
        .await
        .unwrap();

    session
        .stream
        .output
        .send(OutputPackage::SynchronizePlayerPosition(
            SynchronizePlayerPosition::from_player(&session.player),
        ))
        .await
        .unwrap();

    match get_chunk_owner(session.player.get_chunk_position()).await {
        Some(id) => send_session_to_process(id, session).await,

        None => init_process(session).await,
    };
}
