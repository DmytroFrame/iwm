use std::{io::Read, net::TcpStream};

const SEGMENT_BITS: i32 = 0x7F;
const CONTINUE_BIT: i32 = 0x80;

pub(crate) struct PackageReader<'a> {
    stream: &'a TcpStream,
}

impl<'a> PackageReader<'a> {
    pub fn new(stream: &'a mut TcpStream) -> Self {
        PackageReader { stream }
    }

    fn byte(&mut self) -> u8 {
        let mut buf: [u8; 1] = [0; 1];
        self.stream.read(&mut buf).unwrap();
        buf[0]
    }

    pub fn var_int(&mut self) -> i32 {
        let mut value: i32 = 0;
        let mut position: i8 = 0;

        loop {
            let current_byte = self.byte() as i32;
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

    pub fn u16(&mut self) -> u16 {
        u16::from_be_bytes([self.byte(), self.byte()])
    }

    pub fn f32(&mut self) -> f32 {
        f32::from_be_bytes([self.byte(), self.byte(), self.byte(), self.byte()])
    }

    pub fn f64(&mut self) -> f64 {
        f64::from_be_bytes([
            self.byte(),
            self.byte(),
            self.byte(),
            self.byte(),
            self.byte(),
            self.byte(),
            self.byte(),
            self.byte(),
        ])
    }

    pub fn bool(&mut self) -> bool {
        if self.byte() == 0 {
            false
        } else {
            true
        }
    }

    pub fn string(&mut self) -> String {
        let string_length: i32 = self.var_int();
        let mut buf: Vec<u8> = Vec::new();

        for _ in 0..string_length {
            buf.push(self.byte());
        }
        String::from_utf8_lossy(&buf).to_string()
    }
}
