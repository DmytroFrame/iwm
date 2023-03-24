use tokio::{
    io::{AsyncReadExt, ReadHalf},
    net::TcpStream,
};

use crate::net::protocol::utils::const_bits::{CONTINUE_BIT, SEGMENT_BITS};

pub(crate) struct StreamReader<'a> {
    stream: &'a mut ReadHalf<TcpStream>,
}

impl<'a> StreamReader<'a> {
    pub fn new(stream: &'a mut ReadHalf<TcpStream>) -> Self {
        StreamReader { stream }
    }

    async fn byte(&mut self) -> u8 {
        self.stream.read_u8().await.unwrap()
    }

    pub async fn var_int(&mut self) -> i32 {
        let mut value: i32 = 0;
        let mut position: i8 = 0;

        loop {
            let current_byte = self.byte().await as i32;
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

    pub async fn u16(&mut self) -> u16 {
        u16::from_be_bytes([self.byte().await, self.byte().await])
    }

    pub async fn f32(&mut self) -> f32 {
        f32::from_be_bytes([
            self.byte().await,
            self.byte().await,
            self.byte().await,
            self.byte().await,
        ])
    }

    pub async fn f64(&mut self) -> f64 {
        f64::from_be_bytes([
            self.byte().await,
            self.byte().await,
            self.byte().await,
            self.byte().await,
            self.byte().await,
            self.byte().await,
            self.byte().await,
            self.byte().await,
        ])
    }

    pub async fn i64(&mut self) -> i64 {
        i64::from_be_bytes([
            self.byte().await,
            self.byte().await,
            self.byte().await,
            self.byte().await,
            self.byte().await,
            self.byte().await,
            self.byte().await,
            self.byte().await,
        ])
    }

    pub async fn bool(&mut self) -> bool {
        if self.byte().await == 0 {
            false
        } else {
            true
        }
    }

    pub async fn string(&mut self) -> String {
        let string_length: i32 = self.var_int().await;
        let mut buf: Vec<u8> = Vec::new();

        for _ in 0..string_length {
            buf.push(self.byte().await);
        }
        String::from_utf8_lossy(&buf).to_string()
    }
}
