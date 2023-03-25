use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::shared_values::{CURRENT_ONLINE, MAX_ONLINE, PLAYERS_ONLINE};

#[derive(Serialize, Deserialize, Clone)]
pub(crate) struct SamplePlayer {
    name: String,
    id: String,
}

pub async fn get_max_online() -> i32 {
    *MAX_ONLINE.lock().await
}
pub async fn get_current_online() -> i32 {
    *CURRENT_ONLINE.lock().await
}

pub async fn add_online(name: &String, id: &Uuid) {
    let mut players = PLAYERS_ONLINE.lock().await;
    let sample_player = SamplePlayer {
        name: name.to_string(),
        id: id.to_string(),
    };
    players.push(sample_player);

    *CURRENT_ONLINE.lock().await += 1;
}

pub async fn minus_online(name: &String) {
    let mut players = PLAYERS_ONLINE.lock().await;

    for index in 0..players.len() {
        if players[index].name == name.to_string() {
            players.remove(index);
            *CURRENT_ONLINE.lock().await -= 1;
        }
    }
}

pub(crate) async fn get_sample_playes(count: Option<usize>) -> Vec<SamplePlayer> {
    match count {
        Some(count) => {
            let mut players = (*PLAYERS_ONLINE.lock().await.clone()).to_vec();
            players.truncate(count);
            players
        }

        None => (*PLAYERS_ONLINE.lock().await.clone()).to_vec(),
    }
}
