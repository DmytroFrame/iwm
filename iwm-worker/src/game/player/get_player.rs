use uuid::Uuid;

use super::{get_ofline_player_uuid::get_ofline_player_uuid, player_struct::Player};

use crate::storage;

async fn get_player_from_storage(uuid: Uuid) -> Option<Player> {
    let data = storage::read_file(&format!("players/{}.json", uuid.to_string()))
        .await
        .unwrap();

    if data.len() == 0 {
        return None;
    } else {
        return match serde_json::from_str(&data) {
            Ok(player) => Some(player),
            Err(_) => None,
        };
    }
}

pub(crate) async fn get_player(username: &str) -> Player {
    let uuid = get_ofline_player_uuid(username);

    match get_player_from_storage(uuid).await {
        Some(player) => player,

        None => Player {
            entity_id: 7,
            username: username.to_string(),
            uuid,
            gamemode: super::player_struct::Gamemode::Creative,
            position: crate::utils::vec3::Vec3 {
                x: 0.0,
                y: 20.0,
                z: 0.0,
            },
            rotation: crate::utils::vec2::Vec2 { x: 0.0, z: 0.0 },
            on_ground: false,
            health: 20.0,
        },
    }
}
