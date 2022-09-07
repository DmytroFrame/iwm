use crate::network::utils::write;

pub fn set_render_distance(view_distance: i32) -> Vec<u8> {
    let mut buf: Vec<u8> = Vec::new();
    buf.extend(b"\x4C");
    buf.extend(write::int(view_distance));
    buf
}
