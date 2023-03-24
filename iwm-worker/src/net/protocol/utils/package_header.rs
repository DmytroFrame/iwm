use tokio::io::AsyncReadExt;

use super::const_bits::{CONTINUE_BIT, SEGMENT_BITS};
use std::io::Error;

pub(crate) struct PackageHeader {
    pub id: i32,
    pub size: i32,
}

impl PackageHeader {
    pub async fn from_steam<T: tokio::io::AsyncRead + Unpin>(
        stream: &mut T,
    ) -> Result<PackageHeader, Error> {
        Ok(PackageHeader {
            size: read_var_int(stream).await?,
            id: read_var_int(stream).await?,
        })
    }
}

async fn read_var_int<T: tokio::io::AsyncRead + Unpin>(stream: &mut T) -> Result<i32, Error> {
    let mut value: i32 = 0;
    let mut position: i8 = 0;

    loop {
        let current_byte = stream.read_u8().await? as i32;
        value |= (current_byte & SEGMENT_BITS) << position;

        if (current_byte & CONTINUE_BIT) == 0 {
            break;
        };
        position += 7;

        if position >= 32 {
            panic!("VarInt is too big")
        };
    }

    Ok(value)
}
