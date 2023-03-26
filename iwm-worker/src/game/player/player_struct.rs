use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::utils::{vec2::Vec2, vec3::Vec3};

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub(crate) enum Gamemode {
    Survival,
    Creative,
    Adventure,
    Spectator,
    None,
}

impl Gamemode {
    pub fn as_i8(&self) -> i8 {
        match self {
            Gamemode::Survival => 0,
            Gamemode::Creative => 1,
            Gamemode::Adventure => 2,
            Gamemode::Spectator => 3,
            Gamemode::None => -1,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct Player {
    pub entity_id: i32,
    pub username: String,
    pub uuid: Uuid,
    pub gamemode: Gamemode,
    pub position: Vec3<f64>,
    pub rotation: Vec2<f32>,
    pub on_ground: bool,
    pub health: f32,
}

impl Player {
    pub fn get_chunk_position(&self) -> (i32, i32) {
        let x = (self.position.x / 16.0).floor() as i32;
        let z = (self.position.z / 16.0).floor() as i32;
        (x, z)
    }
}
