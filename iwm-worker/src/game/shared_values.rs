use std::collections::HashMap;

use tokio::sync::{mpsc::Sender, Mutex};

use crate::game::online::SamplePlayer;

lazy_static! {
    pub static ref MAX_ONLINE: Mutex<i32> = Mutex::new(20);
    pub static ref CURRENT_ONLINE: Mutex<i32> = Mutex::new(0);
    pub(crate) static ref PLAYERS_ONLINE: Mutex<Vec<SamplePlayer>> = Mutex::new(Vec::new());
    pub static ref VIEW_DISTANCE: Mutex<i32> = Mutex::new(12);
    pub static ref SIMULATION_DISTANCE: Mutex<i32> = Mutex::new(12);
    pub static ref PROCESS_CHANNELS: Mutex<HashMap<String, Sender<i32>>> =
        Mutex::new(HashMap::new());
}
