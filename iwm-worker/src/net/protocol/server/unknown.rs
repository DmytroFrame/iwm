#[derive(Debug)]
pub(crate) struct Unknown {
    pub size: i32,
    pub id: i32,
    pub raw_data: Vec<u8>,
}
