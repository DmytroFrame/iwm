#[macro_use]
extern crate lazy_static;

mod game;
mod logger;
mod net;
mod server;
mod storage;
mod utils;
mod websocket;

#[tokio::main(worker_threads = 1)]
// #[tokio::main]
async fn main() {
    // console_subscriber::init();

    websocket::init_ws_server(3000).await;
    server::init_server(25565).await;
}
