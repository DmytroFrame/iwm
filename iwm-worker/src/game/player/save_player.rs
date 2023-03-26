use super::player_struct::Player;
use crate::storage;

pub(crate) async fn save_player(player: &Player) {
    let data = serde_json::to_string(&player).unwrap();

    storage::write_file(&format!("players/{}.json", player.uuid.to_string()), data)
        .await
        .unwrap();
}
