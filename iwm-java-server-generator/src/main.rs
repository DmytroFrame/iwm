#[macro_use]
extern crate lazy_static;

mod grpc_server;
mod java_server;

use grpc_server::grpc_server;
use java_server::java_server;

#[tokio::main]
async fn main() {
    java_server().await;

    grpc_server().await;
}
