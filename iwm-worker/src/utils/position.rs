#[derive(Clone, Copy)]
pub(crate) struct Position {
    x: i64,
    y: i64,
    z: i64,
}

impl Position {
    pub fn new(x: i64, y: i64, z: i64) -> Position {
        Position { x, y, z }
    }

    pub fn as_i64(&self) -> i64 {
        ((self.x & 0x3FFFFFF) << 38) | ((self.z & 0x3FFFFFF) << 12) | (self.y & 0xFFF)
    }
}
