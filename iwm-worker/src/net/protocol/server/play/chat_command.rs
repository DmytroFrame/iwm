use crate::net::protocol::utils::buffer_reader::BufferReader;

#[derive(Debug)]
pub(crate) struct ChatCommand {
    pub message: String,
}

impl ChatCommand {
    pub fn from_buffer(buf: Vec<u8>) -> ChatCommand {
        let mut reader = BufferReader::new(buf);

        ChatCommand {
            message: reader.string(),
        }
    }
}
