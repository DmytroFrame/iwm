use super::client::{keep_alive::KeepAlive, set_center_chunk::SetCenterChunk};

#[derive(Debug)]
pub(crate) enum OutputPackage {
    SetCenterChunk(SetCenterChunk),
    KeepAlive(KeepAlive),
}

pub(crate) async fn output_package_handle(package: OutputPackage) -> Vec<u8> {
    match package {
        OutputPackage::SetCenterChunk(payload) => payload.to_bytes(),
        OutputPackage::KeepAlive(payload) => payload.to_bytes(),
    }
}
