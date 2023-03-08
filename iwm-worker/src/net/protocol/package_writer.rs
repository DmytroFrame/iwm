const SEGMENT_BITS: i32 = 0x7F;
const CONTINUE_BIT: i32 = 0x80;

pub(crate) struct PackageWriter {
    buf: Vec<u8>,
}

impl PackageWriter {
    pub fn new() -> Self {
        PackageWriter { buf: Vec::new() }
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

    pub fn long(&mut self, value: i64) {
        self.buf.extend(value.to_be_bytes());
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

// fn unsigned_right_shift(value: i32, n: i32) -> u32 {
//     let value_as_u32: u32 = {
//         let bytes = value.to_be_bytes();
//         u32::from_be_bytes(bytes)
//     };
//     return value_as_u32 >> n;
// }
