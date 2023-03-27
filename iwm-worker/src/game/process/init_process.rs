use std::time::Duration;

use tokio::{sync::mpsc::Receiver, time::Instant};

use crate::{game::online, logger::Logger};

use super::{
    game_process::game_process, init_player_session::PlayerSession,
    process_channels::process_registration,
};

pub(super) struct Process {
    pub players: Vec<PlayerSession>,
    pub events: Vec<String>,
    pub chunks: Vec<String>,
    pub session_queue: Receiver<PlayerSession>,
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

    pub async fn is_there_new_session(&mut self) {
        match self.session_queue.try_recv() {
            Ok(session) => {
                self.players.push(session);

                println!("\nNew session!\n");
            }

            Err(_) => {}
        };
    }
}

pub(crate) async fn init_process(player_session: PlayerSession) {
    let session_queue = process_registration(player_session.player.entity_id).await;

    let mut process = Process {
        players: vec![player_session],
        events: vec![],
        chunks: vec![],
        session_queue,
    };

    loop {
        // let start  = Instant::now();
        // println!("game_process");
        game_process(&mut process).await;

        // println!("is_there_new_session");
        process.is_there_new_session().await;

        // println!("is_all_disconnected");
        if process.is_all_disconnected() {
            Logger::new("Process").info("Process is ended");
            online::minus_online(&process.players[0].player.username).await;
            break;
        }

        // Logger::new("Process").info(&format!("Process loop time {:?}", Instant::elapsed(&start)));
        tokio::time::sleep(Duration::from_millis(50)).await;
    }
}
