use std::str::FromStr;
use uuid::Uuid;

pub fn get_ofline_player_uuid(username: &str) -> Uuid {
    let digest = md5::compute(format!("OfflinePlayer:{}", username));
    let str_hex = format!("{:x}", digest);
    let str_uuid = format!(
        "{}-{}-3{}-9{}-{}",
        &str_hex[..8],
        &str_hex[8..12],
        &str_hex[13..16],
        &str_hex[17..20],
        &str_hex[20..]
    );

    Uuid::from_str(&str_uuid).unwrap()
}
