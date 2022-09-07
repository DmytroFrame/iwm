use std::{io::Read, net::TcpStream};

const SEGMENT_BITS: i32 = 0x7F;
const CONTINUE_BIT: i32 = 0x80;

pub fn byte(mut stream: &TcpStream) -> u8 {
    let mut buf = [0; 1];
    stream.read(&mut buf).expect("Ніхуя ти не прочитаеш!");
    // println!("READ:{} ", buf[0]);
    buf[0]
}

pub fn int(stream: &TcpStream) -> i32 {
    let mut value: i32 = 0;
    let mut position: i8 = 0;

    loop {
        let current_byte = self::byte(&stream) as i32;
        value |= (current_byte & SEGMENT_BITS) << position;

        if (current_byte & CONTINUE_BIT) == 0 {
            break;
        };
        position += 7;

        if position >= 32 {
            panic!("VarInt is too big")
        };
    }

    return value;
}

pub fn unsigned_short(stream: &TcpStream) -> u16 {
    u16::from_be_bytes([self::byte(&stream), self::byte(&stream)])
}

pub fn double(stream: &TcpStream) -> f64 {
    f64::from_be_bytes([
        self::byte(stream),
        self::byte(stream),
        self::byte(stream),
        self::byte(stream),
        self::byte(stream),
        self::byte(stream),
        self::byte(stream),
        self::byte(stream),
    ])
}

pub fn string(stream: &TcpStream) -> String {
    let string_length: i32 = self::int(&stream);
    let mut buf: Vec<u8> = Vec::new();
    for _ in 0..string_length {
        buf.push(self::byte(&stream));
    }
    String::from_utf8_lossy(&buf).to_string()
}
