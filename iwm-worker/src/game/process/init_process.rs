use std::time::Duration;

use tokio::{sync::mpsc::Receiver, time::Instant};

use crate::{
    game::{event::manager::EventMenager, online},
    logger::Logger, websocket,
};

use super::{
    game_process::game_process, init_player_session::PlayerSession,
    process_channels::process_registration,
};

#[derive(Debug)]
pub(crate) struct Process {
    pub players: Vec<PlayerSession>,
    // pub events: EventMenager,
    // pub chunks: Vec<String>,
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

    pub async fn is_there_new_session(&mut self, event: &mut EventMenager) {
        match self.session_queue.try_recv() {
            Ok(session) => {
                let id = session.player.entity_id;

                self.players.push(session);
                println!("New Player Event");
                event.add_event(crate::game::event::events::Events::PlayerJoin, id);
                // self.

                println!("\nNew session!\n");
            }

            Err(_) => {}
        };
    }

    pub fn get_palyer_by_id(&self, id: i32) -> Option<&PlayerSession> {
        for player in &self.players {
            if player.player.entity_id == id {
                return Some(player);
            }
        }
        return None;
    }
}

pub(crate) async fn init_process(player_session: PlayerSession) {
    let session_queue: Receiver<PlayerSession> = process_registration(player_session.player.entity_id).await;

    let mut event = EventMenager::new();
    let mut process = Process {
        players: vec![player_session],
        // chunks: vec![],
        session_queue,
    };

    process.players[0].init_event_handlers(&mut event);

    loop {
        // let start  = Instant::now();
        // println!("game_process");
        game_process(&mut process, &mut event).await;

        event.run_all(&mut process).await;

        // println!("is_there_new_session");
        process.is_there_new_session(&mut event).await;

        // println!("is_all_disconnected");
        if process.is_all_disconnected() {
            Logger::new("Process").info("Process is ended");
            online::minus_online(&process.players[0].player.username).await;
            break;
        }

        websocket::send_ws_message(format!("{:#?}", &process)).await;

        // Logger::new("Process").info(&format!("Process loop time {:?}", Instant::elapsed(&start)));
        tokio::time::sleep(Duration::from_millis(50)).await;
    }
}
