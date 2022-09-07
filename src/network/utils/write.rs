use std::{io::Write, net::TcpStream};

const SEGMENT_BITS: i32 = 0x7F;
const CONTINUE_BIT: i32 = 0x80;

fn unsigned_right_shift(value: i32, n: i32) -> u32 {
    let value_as_u32: u32 = {
        let bytes = value.to_be_bytes();
        u32::from_be_bytes(bytes)
    };
    return value_as_u32 >> n;
}

pub fn bytes(mut stream: &TcpStream, buf: Vec<u8>) {
    let mut new_buf = Vec::new();
    new_buf.extend(self::int(buf.len() as i32));
    new_buf.extend(buf);
    // print!("WRITE BYTES {:?}", new_buf);
    stream.write(&new_buf[..]).unwrap();
}

pub fn int(mut value: i32) -> Vec<u8> {
    let mut buf: Vec<u8> = Vec::new();

    loop {
        if (value & !SEGMENT_BITS) == 0 {
            buf.push(value as u8);
            return buf;
        }
        buf.push(((value & SEGMENT_BITS) | CONTINUE_BIT) as u8);

        value = unsigned_right_shift(value, 7) as i32;
    }
}

pub fn string(data: String) -> Vec<u8> {
    let mut buf = Vec::new();
    let buf_data = data.bytes();
    buf.extend(self::int(buf_data.len() as i32));
    buf.extend(buf_data);
    buf
}
