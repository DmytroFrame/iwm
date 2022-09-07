pub fn set_player_position(x: f64, y: f64, z: f64) -> Vec<u8> {
    let mut buf = Vec::new();
    buf.push(57);
    buf.extend(x.to_be_bytes());
    buf.extend(y.to_be_bytes());
    buf.extend(z.to_be_bytes());
    buf.extend(b"\x41\x17\x33\xe6\x40\xf4\xc6\xfc\x00\x01\x00");
    buf
}
