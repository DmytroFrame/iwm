use crate::logger::Logger;
use futures_util::{SinkExt, StreamExt};
use std::net::SocketAddr;
use tokio::net::{TcpListener, TcpStream};
use tokio::sync::mpsc::{channel, Receiver, Sender};
use tokio::sync::Mutex;
use tokio_tungstenite::accept_async;
use tokio_tungstenite::tungstenite::protocol::Message;

lazy_static! {
    static ref TX_WS_MESSAGES: Mutex<Sender<String>> = Mutex::new(channel(1).0);
    static ref RX_WS_MESSAGES: Mutex<Receiver<String>> = Mutex::new(channel(1).1);
}

pub async fn send_ws_message(message: String) {
    let _ = TX_WS_MESSAGES.lock().await.try_send(message);
}

pub async fn init_ws_server(port: u16) {
    let (tx, rx) = channel(20);
    *TX_WS_MESSAGES.lock().await = tx;
    *RX_WS_MESSAGES.lock().await = rx;

    let addr: SocketAddr = format!("127.0.0.1:{}", port).parse().unwrap();
    let listener = TcpListener::bind(&addr).await.unwrap();

    Logger::new("WS:Server").info(&format!("WebSocket server started on: {}", addr));

    tokio::spawn(async move {
        loop {
            let (stream, _) = listener.accept().await.unwrap();
            tokio::spawn(handle_ws_client(stream));
        }
    });
}

async fn handle_ws_client(stream: TcpStream) {
    if let Err(e) = process_ws_client(stream).await {
        Logger::new("WS:Server").error(&format!("Error processing client: {}", e));
    }
}

async fn process_ws_client(stream: TcpStream) -> Result<(), Box<dyn std::error::Error>> {
    let addr = stream.peer_addr()?;
    Logger::new("WS:Server").info(&format!("New WebSocket connection: {}", addr));

    let ws_stream = accept_async(stream).await?;
    let (mut write, _) = ws_stream.split();

    let mut rx = RX_WS_MESSAGES.lock().await;

    loop {
        match rx.recv().await {
            None => {}
            Some(msg) => {
                let message = Message::Text(msg);
                write.send(message).await?;
            }
        }
    }
}
