pub fn get_delta_position(curr: f64, prew: f64) -> i16 {
    ((curr.floor() as i64 * 32 - prew.floor() as i64 * 32) * 120) as i16
}
