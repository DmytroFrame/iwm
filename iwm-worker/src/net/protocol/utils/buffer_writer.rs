use uuid::Uuid;

use crate::net::protocol::utils::const_bits::{CONTINUE_BIT, SEGMENT_BITS};

pub(crate) struct BufferWriter {
    buf: Vec<u8>,
}

impl BufferWriter {
    pub fn new() -> Self {
        BufferWriter { buf: Vec::new() }
    }

    pub fn byte(&mut self, byte: u8) {
        self.buf.push(byte);
    }

    pub fn bytes(&mut self, byte: &[u8]) {
        self.buf.extend(byte);
    }

    pub fn var_int(&mut self, mut value: i32) {
        loop {
            if (value & !SEGMENT_BITS) == 0 {
                self.buf.push(value as u8);
                return;
            }

            self.buf.push(((value & SEGMENT_BITS) | CONTINUE_BIT) as u8);
            value = (u32::from_be_bytes(value.to_be_bytes()) >> 7) as i32
        }
    }

    pub fn string(&mut self, data: String) {
        let buf_data = data.bytes();
        self.var_int(buf_data.len() as i32);
        self.buf.extend(buf_data);
    }

    pub fn uuid(&mut self, uuid: Uuid) {
        self.buf.extend(uuid.as_bytes());
    }

    pub fn i32(&mut self, value: i32) {
        self.buf.extend(value.to_be_bytes());
    }

    pub fn i64(&mut self, value: i64) {
        self.buf.extend(value.to_be_bytes());
    }

    pub fn i8(&mut self, value: i8) {
        self.buf.extend(value.to_be_bytes());
    }

    pub fn bool(&mut self, value: bool) {
        if value {
            self.buf.push(0x01);
        } else {
            self.buf.push(0x00);
        }
    }

    pub fn build(&mut self) -> Vec<u8> {
        let mut result_buf = Vec::new();
        let current_size = self.buf.len();

        result_buf.extend(int(current_size as i32));
        result_buf.extend(&self.buf);

        result_buf
    }
}

fn int(mut value: i32) -> Vec<u8> {
    let mut buf: Vec<u8> = Vec::new();

    loop {
        if (value & !SEGMENT_BITS) == 0 {
            buf.push(value as u8);
            return buf;
        }
        buf.push(((value & SEGMENT_BITS) | CONTINUE_BIT) as u8);

        value = (u32::from_be_bytes(value.to_be_bytes()) >> 7) as i32
    }
}
