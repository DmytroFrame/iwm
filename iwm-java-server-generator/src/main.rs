#[macro_use]
extern crate lazy_static;

mod java_server;
mod grpc_server;

use java_server::java_server;
use grpc_server::grpc_server;



#[tokio::main]
async fn main() {
    java_server().await;

    grpc_server().await;
}