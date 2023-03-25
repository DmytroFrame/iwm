use crate::net::protocol::utils::buffer_reader::BufferReader;

#[derive(Debug)]
pub(crate) struct ChatMessage {
    pub message: String,
}

impl ChatMessage {
    pub fn from_buffer(buf: Vec<u8>) -> ChatMessage {
        let mut reader = BufferReader::new(buf);

        ChatMessage {
            message: reader.string(),
        }
    }
}
