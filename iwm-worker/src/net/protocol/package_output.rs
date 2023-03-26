use super::client::play::{
    chunk_data_and_update_light::ChunkDataAndUpdateLight, keep_alive::KeepAlive,
    set_center_chunk::SetCenterChunk, set_render_distance::SetRenderDistance,
    synchronize_player_position::SynchronizePlayerPosition,
};

#[derive(Debug, PartialEq)]
pub(crate) enum OutputPackage {
    SetCenterChunk(SetCenterChunk),
    KeepAlive(KeepAlive),
    ChunkDataAndUpdateLight(ChunkDataAndUpdateLight),
    SetRenderDistance(SetRenderDistance),
    SynchronizePlayerPosition(SynchronizePlayerPosition),
}

pub(crate) async fn output_package_handle(package: OutputPackage) -> Vec<u8> {
    match package {
        OutputPackage::SetCenterChunk(payload) => payload.to_bytes(),

        OutputPackage::KeepAlive(payload) => payload.to_bytes(),

        OutputPackage::ChunkDataAndUpdateLight(payload) => payload.to_bytes(),

        OutputPackage::SetRenderDistance(payload) => payload.to_bytes(),

        OutputPackage::SynchronizePlayerPosition(payload) => payload.to_bytes(),
    }
}
