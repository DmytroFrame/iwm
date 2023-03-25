use crate::net::protocol::client::play::chunk_data_and_update_light::ChunkDataAndUpdateLight;

const FLAT_CHUNK: &[u8; 9091] = include_bytes!("flat_chunk.bin");

pub(crate) fn get_flat_chunk(x: i32, z: i32) -> ChunkDataAndUpdateLight {
    ChunkDataAndUpdateLight {
        raw_data: (&FLAT_CHUNK[9..]).to_vec(),
        x,
        z,
    }
}
