use serde::{Deserialize, Serialize};
#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct Vec2<T> {
    pub x: T,
    pub z: T,
}
