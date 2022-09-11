use std::io::{prelude::*, Result};
use std::net::TcpStream;
use std::thread;
mod consts;
mod network;
mod server;
mod status;

use network::utils::{read, write};
use status::online;

fn main() {
    println!(
        r###"
   ██╗██╗    ██╗███╗   ███╗TM   DmytroFrame Software® inc 2022
   ██║██║    ██║████╗ ████║     Це майнкрафт сервер
   ██║██║ █╗ ██║██╔████╔██║     v0.0.0.0.23.12.5
   ██║██║███╗██║██║╚██╔╝██║
   ██║╚███╔███╔╝██║ ╚═╝ ██║     HOST:  localhost:8080
   ╚═╝ ╚══╝╚══╝ ╚═╝     ╚═╝
       "###
    );
    server::start(8080);
}

fn network_handle(mut stream: TcpStream, _id: i32, _size: i32) -> Result<()> {
    let protocol_version = read::int(&stream);
    let adderss = read::string(&stream);
    let port = read::unsigned_short(&stream);
    let status = read::int(&stream);
    println!(
        "CONNECT INFO: {} {} {} {}",
        protocol_version, adderss, port, status
    );
    if status == 1 {
        skip_lisen(&stream);
        status_response(&stream, protocol_version);
        skip_lisen(&stream);
        stream
            .write(b"\t\x01\x00\x00\x00\x00\x00\xd4\xedQ")
            .unwrap();
    } else {
        skip_lisen(&stream);
        write::bytes(&stream, b"\x02\xaa\x31\xd6\xfb\xe4\x69\x3a\xe4\x9b\x7f\x97\xc6\x3c\x75\x4c\x88\x0b\x44\x6d\x79\x74\x72\x6f\x46\x72\x61\x6d\x65\x00".to_vec());
        write::bytes(&stream, consts::LOGIN.to_vec());

        write::bytes(&stream, b"\x4a\x01".to_vec());
        write::bytes(&stream, b"\x39\x3f\xd7\xed\xf7\xe8\x0b\x69\x30\x40\x00\x00\x00\x00\x00\x00\x00\x3f\xe8\x2f\x52\x41\x4d\x84\x7c\x41\x17\x33\xe6\x40\xf4\xc6\xfc\x00\x01\x00".to_vec());
        write::bytes(&stream, create_level_1(0, 0));
        write::bytes(&stream, create_level_2(0, 1));
        write::bytes(&stream, set_render_distance(10));
        write::bytes(&stream, set_player_position(6.7, 2.0, 7.2));

        unsafe {
            NEW_LEVEL_COUNTER = 1;
            OLD_LEVEL_COUNTER = 1;
        }

        write::bytes(&stream, create_level_2(0, 1));

        loop {
            write::bytes(&stream, set_health(20.0, 15, 0.0));
            skip_lisen(&stream);
        }
    }

    Ok(())
}

static mut NEW_LEVEL_COUNTER: i32 = 1;

fn detect_new_level(stream: &TcpStream, z: f64) {
    unsafe {
        if z > (3 + (NEW_LEVEL_COUNTER * 16)) as f64 {
            NEW_LEVEL_COUNTER += 1;
            write::bytes(stream, create_level_2(0, NEW_LEVEL_COUNTER));
        }
    }
}

static mut OLD_LEVEL_COUNTER: i32 = 1;

fn detect_old_level(stream: &TcpStream, z: f64) {
    unsafe {
        if z > (11 + (OLD_LEVEL_COUNTER * 16)) as f64 {
            OLD_LEVEL_COUNTER += 1;
            write::bytes(stream, create_unload_chunk(0, OLD_LEVEL_COUNTER - 2));
        }
    }
}

fn skip_lisen(mut stream: &TcpStream) {
    let size = read::int(stream);
    let id = read::int(stream) as u8;
    // println!("SKIP size: {} with id: {} OR hexId: 0x{:X}", size, id, id);
    match id {
        20 => {
            let _x = read::double(stream);
            let y = read::double(stream);
            let z = read::double(stream);
            let on_ground = {
                if read::byte(stream) == 1 {
                    true
                } else {
                    false
                }
            };
            // print!(" Position\tX: {}\tFeet Y: {},\tZ: {}\tOn Ground: {}\n", x, y, z, on_ground);

            if on_ground {
                detect_new_level(stream, z);
                detect_old_level(stream, z);
            }
            if y < -100.0 {
                write::bytes(stream, set_health(0.0, 20, 0.0));
                thread::sleep_ms(1000);
                unsafe {
                    if OLD_LEVEL_COUNTER - 1 > MAX_SCORE {
                        MAX_SCORE = OLD_LEVEL_COUNTER - 1;
                    }
                };
                write::bytes(
                    stream,
                    create_message(
                        "Ти програв, твій рівень: ".to_owned()
                            + unsafe { &(OLD_LEVEL_COUNTER - 1).to_string() },
                    ),
                );
                stream.flush().unwrap();
                panic!();
            }
        }

        _ => {
            for _ in 1..size {
                let _read = read::byte(stream);
            }
        }
    }
}

#[allow(dead_code)]
fn create_level(x: i32, z: i32) -> Vec<u8> {
    let mut buf_level = Vec::new();
    buf_level.push(33);
    buf_level.extend(i32::from(x).to_be_bytes());
    buf_level.extend(i32::from(z).to_be_bytes());
    buf_level.extend(consts::MAP_LEVEL_WITH_LIGHT);
    buf_level
}

#[allow(dead_code)]
fn create_level_1(x: i32, z: i32) -> Vec<u8> {
    let mut buf_level = Vec::new();
    buf_level.push(33);
    buf_level.extend(i32::from(x).to_be_bytes());
    buf_level.extend(i32::from(z).to_be_bytes());
    buf_level.extend(consts::LEVEL_1);
    buf_level
}
#[allow(dead_code)]
fn create_level_2(x: i32, z: i32) -> Vec<u8> {
    let mut buf_level = Vec::new();
    buf_level.push(33);
    buf_level.extend(i32::from(x).to_be_bytes());
    buf_level.extend(i32::from(z).to_be_bytes());
    buf_level.extend(consts::LEVEL_2);
    buf_level
}

#[allow(dead_code)]
fn create_level_empty(x: i32, z: i32) -> Vec<u8> {
    let mut buf_level = Vec::new();
    buf_level.push(33);
    buf_level.extend(i32::from(x).to_be_bytes());
    buf_level.extend(i32::from(z).to_be_bytes());
    buf_level.extend(consts::LEVEL_EMPTY);
    buf_level
}

fn create_message(msg: String) -> Vec<u8> {
    let mut buf = Vec::new();
    let message = r###"{ "text": ""###.to_owned() + &msg + r###"", "bold": true }"###;
    buf.push(25);
    buf.extend(write::string(message.to_string()));
    buf
}

static mut MAX_SCORE: i32 = 0;

fn status_response(stream: &TcpStream, protocol: i32) {
    let mut string = String::new();
    string.push_str(r###"{ "version": { "name": "2", "protocol": "###);
    string.push_str(&protocol.to_string());
    string.push_str(r###" }, "players": { "max": -7, "online": "###);
    string.push_str(&(online::get() - 1).to_string());
    string.push_str(r###", "sample": [ { "name": "DmytroFrame", "id": "4566e69f-c907-48ee-8d71-d7ba5aa00d20" } ] }, "description": { "text": "Infinity parkour! Record level: "###);
    string.push_str(unsafe { &MAX_SCORE.to_string() });
    string.push_str(r###""}, "favicon": "data:image/png;base64,<>", "previewsChat": false }"###);

    let mut buf = Vec::new();
    buf.push(0);
    buf.extend(write::string(string));
    write::bytes(stream, buf);
}

#[allow(dead_code)]
fn create_game_event(event: u8, value: u8) -> Vec<u8> {
    let mut buf = Vec::new();
    buf.push(29);
    buf.push(event);
    buf.extend(f32::from(value).to_be_bytes());
    // println!("SEND EVENT: {:?}", buf);
    buf
}

fn set_health(health: f32, food: i32, food_saturation: f32) -> Vec<u8> {
    let mut buf = Vec::new();
    buf.push(85);
    buf.extend(f32::from(health).to_be_bytes());
    buf.extend(write::int(food));
    buf.extend(f32::from(food_saturation).to_be_bytes());
    buf
}

fn set_player_position(x: f64, y: f64, z: f64) -> Vec<u8> {
    let mut buf = Vec::new();
    buf.push(57);
    buf.extend(x.to_be_bytes());
    buf.extend(y.to_be_bytes());
    buf.extend(z.to_be_bytes());
    buf.extend(b"\x41\x17\x33\xe6\x40\xf4\xc6\xfc\x00\x01\x00");
    buf
}

fn create_unload_chunk(x: i32, z: i32) -> Vec<u8> {
    let mut unload_chunk = Vec::new();
    unload_chunk.push(28);
    unload_chunk.extend(x.to_be_bytes());
    unload_chunk.extend(z.to_be_bytes());
    unload_chunk
}

fn set_render_distance(view_distance: i32) -> Vec<u8> {
    let mut buf: Vec<u8> = Vec::new();
    buf.extend(b"\x4C");
    buf.extend(write::int(view_distance));
    buf
}

// fn play_login(stream: &TcpStream) {
//     let mut buf: Vec<u8> = Vec::new();

//     buf.extend(i32::from(1).to_be_bytes());
//     buf.extend(u8::from(1).to_be_bytes());
//     buf.push(1);

// }
