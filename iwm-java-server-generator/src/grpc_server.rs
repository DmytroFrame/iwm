use tonic::{transport::Server, Request, Response, Status};

use chunk_generator::chunk_generator_server::{ChunkGenerator, ChunkGeneratorServer};
use chunk_generator::{GetChunkRequest, GetChunkResponse};

pub mod chunk_generator {
    tonic::include_proto!("chunk_generator");
}

#[derive(Default)]
pub struct ChunkGeneratorService {}

#[tonic::async_trait]
impl ChunkGenerator for ChunkGeneratorService {
    async fn get_chunk(
        &self,
        request: Request<GetChunkRequest>,
    ) -> Result<Response<GetChunkResponse>, Status> {
        println!("Got a request from {:?}", request.remote_addr());

        let reply = chunk_generator::GetChunkResponse {
            data: vec![2; 17000],
        };
        Ok(Response::new(reply))
    }
}

pub async fn grpc_server() -> Result<(), Box<dyn std::error::Error>> {
    let addres = "127.0.0.1:7820".parse().unwrap();
    let service = ChunkGeneratorService::default();

    println!("GreeterServer listening on {}", addres);

    Server::builder()
        .add_service(ChunkGeneratorServer::new(service))
        .serve(addres)
        .await?;

    Ok(())
}
