use crate::game::shared_values::PROCESS_CHANNELS;
use tokio::sync::mpsc::{channel, Receiver, Sender};

use super::init_player_session::PlayerSession;

pub(crate) async fn process_registration(id: i32) -> Receiver<PlayerSession> {
    let (tx, rx): (Sender<PlayerSession>, Receiver<PlayerSession>) = channel(1);

    let mut map = PROCESS_CHANNELS.lock().await;
    map.entry(id).or_insert(tx);

    rx
}

pub(crate) async fn send_session_to_process(id: i32, session: PlayerSession) {
    PROCESS_CHANNELS
        .lock()
        .await
        .get(&id)
        .unwrap()
        .send(session)
        .await;
}
