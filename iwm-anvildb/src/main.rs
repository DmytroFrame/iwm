// use tonic::{transport::Server, Request, Response, Status};

// use anvildb::anvildb_server::{Anvildb, AnvildbServer};
// use anvildb::{Empty, GetPlayerByUuid, Player};

// mod anvildb {
//     tonic::include_proto!("anvildb");
// }

// // #[derive(Default)]
// // pub struct MyGreeter {}

// // #[tonic::async_trait]
// // impl Anvildb for MyGreeter {
// //     async fn get_player(
// //         &self,
// //         request: Request<GetPlayerByUuid>,
// //     ) -> Result<Response<Player>, Status> {
// //     }

// //     async fn set_player(&self, request: Request<Player>) -> Result<Response<Empty>, Status> {
// //         Ok(Response::new(Empty {}))
// //     }

// //     // ) -> Result<Response<HelloReply>, Status> {
// //     //     println!("Got a request from {:?}", request.remote_addr());

// //     //     let reply = hello_world::HelloReply {
// //     //         message: format!("Hello {}!", request.into_inner().name),
// //     //     };
// //     //     Ok(Response::new(reply))
// //     // }
// // }

// // #[tokio::main]
// // async fn main() -> Result<(), Box<dyn std::error::Error>> {
// //     let addr = "[::1]:50051".parse().unwrap();
// //     let greeter = MyGreeter::default();

// //     println!("GreeterServer listening on {}", addr);

// //     Server::builder()
// //         .add_service(AnvildbServer::new(greeter))
// //         .serve(addr)
// //         .await?;

// //     Ok(())
// // }

// mod service;
// mod storage;

#[tokio::main(worker_threads = 1)]
async fn main() {

    // let path = String::from("players/kek.json");
    // let data = String::from("hi world");

    // storage::write_file(&path, data).await.unwrap();

    // let result =  storage::read_file(&path).await.unwrap();
    // println!("{}", result);
}
