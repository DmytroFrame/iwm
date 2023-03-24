use crate::net::protocol::utils::const_bits::{CONTINUE_BIT, SEGMENT_BITS};

pub(crate) struct BufferReader {
    buf: Vec<u8>,
}

impl BufferReader {
    pub fn new(buf: Vec<u8>) -> Self {
        BufferReader { buf }
    }

    pub fn byte(&mut self) -> u8 {
        self.buf.remove(0)
    }

    pub fn bytes(&mut self, size: usize) -> Vec<u8> {
        let mut buf = Vec::new();
        for _ in 0..size {
            buf.push(self.byte())
        }
        buf
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

    pub fn i64(&mut self) -> i64 {
        i64::from_be_bytes([
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

    pub fn uuid(&mut self) -> String {
        let mut result = String::new();

        for byte in self.bytes(16) {
            result.push_str(&format!("{:x}", byte));
        }

        result
    }
}
