use super::client::{
    chunk_data_and_update_light::ChunkDataAndUpdateLight, keep_alive::KeepAlive,
    set_center_chunk::SetCenterChunk,
};

#[derive(Debug, PartialEq)]
pub(crate) enum OutputPackage {
    SetCenterChunk(SetCenterChunk),
    KeepAlive(KeepAlive),
    ChunkDataAndUpdateLight(ChunkDataAndUpdateLight),
}

pub(crate) async fn output_package_handle(package: OutputPackage) -> Vec<u8> {
    match package {
        OutputPackage::SetCenterChunk(payload) => payload.to_bytes(),
        OutputPackage::KeepAlive(payload) => payload.to_bytes(),
        OutputPackage::ChunkDataAndUpdateLight(payload) => payload.to_bytes(),
    }
}
