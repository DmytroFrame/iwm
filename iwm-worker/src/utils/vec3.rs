use serde::{Deserialize, Serialize};
#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct Vec3<T> {
    pub x: T,
    pub y: T,
    pub z: T,
}
