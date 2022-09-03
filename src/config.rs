use std::{fs::File, io::Read};

use serde::Deserialize;



#[derive(Deserialize)]
pub struct ServerConfig {
    pub server_port: u16,
    pub server_ip: String,
    pub max_players: i32,
    pub motd: String
}


pub fn read() -> ServerConfig {
    let mut file = File::open("config/server.json").unwrap();
    let mut buffer = String::new();

    file.read_to_string(&mut buffer).unwrap();
    serde_json::from_str(&mut buffer).unwrap()
}


