// #[macro_use]
// extern crate lazy_static;

mod game;
mod logger;
mod net;
mod server;
mod utils;

#[tokio::main(worker_threads = 1)]
// #[tokio::main]
async fn main() {
    // console_subscriber::init();
    server::init_server(25565).await;
}
