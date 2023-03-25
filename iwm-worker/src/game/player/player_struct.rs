use uuid::Uuid;

#[derive(Debug, Clone, Copy)]
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

#[derive(Debug)]
pub(crate) struct Vec3<T> {
    pub x: T,
    pub y: T,
    pub z: T,
}

#[derive(Debug)]
pub(crate) struct Vec2<T> {
    pub x: T,
    pub z: T,
}

#[derive(Debug)]
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
