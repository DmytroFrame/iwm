// mod config;

// use std::{fs::File, io::Read};
// use serde::De&serializ&&&e;

// #[derive(Deserialize)]
// struct User {
//     server_port: u16,
// }

// use std::net::TcpListener;

// fn main() {

// let ser = config::read();
// println!("{:?}", f64::MAX.to_string().len());

// }

// use std::fs::File;
use std::io::prelude::*;
use std::net::{TcpListener, TcpStream};
use std::thread;
mod consts;

const SEGMENT_BITS: i32 = 0x7F;
const CONTINUE_BIT: i32 = 0x80;

fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:8080")?;
    println!(
        r###"
    ██╗██╗    ██╗███╗   ███╗TM   DmytroFrame Software® inc 2022
    ██║██║    ██║████╗ ████║     Це майнкрафт сервер
    ██║██║ █╗ ██║██╔████╔██║     v0.0.0.0.23.12.2
    ██║██║███╗██║██║╚██╔╝██║
    ██║╚███╔███╔╝██║ ╚═╝ ██║     HOST:  localhost:8080
    ╚═╝ ╚══╝╚══╝ ╚═╝     ╚═╝
"###
    );

    // accept connections and process them serially
    for stream in listener.incoming() {
        thread::spawn(move || {
            handle_client(stream.expect("#########Хуйня на треді!########"));
            // println!("END {:?}", thread::current().id())
        });
    }
    Ok(())
}

fn handle_client(mut stream: TcpStream) {
    // let mut count = -1;

    let size = read_int(&stream);
    let id = read_int(&stream);

    if id == 122 && size > 200 {
        stream.flush().unwrap();
        // println!("Fuck final {}", id);
        return;
    }

    network_handle(&stream, id, size);

    stream.flush().unwrap();
    // println!("final {}", id);
}

fn read_byte(mut stream: &TcpStream) -> u8 {
    let mut buf = [0; 1];
    stream.read(&mut buf).expect("Ніхуя ти не прочитаеш!");
    // println!("READ:{} ", buf[0]);
    buf[0]
}

fn read_int(stream: &TcpStream) -> i32 {
    let mut value: i32 = 0;
    let mut position: i8 = 0;

    loop {
        let current_byte = read_byte(&stream) as i32;
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

fn read_string(stream: &TcpStream) -> String {
    let string_length: i32 = read_int(&stream);
    let mut buf: Vec<u8> = Vec::new();
    for _ in 0..string_length {
        buf.push(read_byte(&stream));
    }
    String::from_utf8_lossy(&buf).to_string()
}

fn read_unsigned_short(stream: &TcpStream) -> u16 {
    u16::from_be_bytes([read_byte(&stream), read_byte(&stream)])
}

fn read_double(stream: &TcpStream) -> f64 {
    f64::from_be_bytes([read_byte(stream), read_byte(stream), read_byte(stream), read_byte(stream), read_byte(stream), read_byte(stream), read_byte(stream), read_byte(stream)])
}


fn unsigned_right_shift(value: i32, n: i32) -> u32 {
    let value_as_u32: u32 = {
        let bytes = value.to_be_bytes();
        u32::from_be_bytes(bytes)
    };
    return value_as_u32 >> n;
}

fn write_bytes(mut stream: &TcpStream, buf: Vec<u8>) {
    let mut new_buf = Vec::new();
    new_buf.extend(write_int(buf.len() as i32));
    new_buf.extend(buf);
    // print!("WRITE BYTES {:?}", new_buf);
    stream.write(&new_buf[..]).expect("!");
}

fn write_int(mut value: i32) -> Vec<u8> {
    let mut buf: Vec<u8> = Vec::new();

    loop {
        if (value & !SEGMENT_BITS) == 0 {
            buf.push(value as u8);
            return buf;
        }
        buf.push(((value & SEGMENT_BITS) | CONTINUE_BIT) as u8);

        value = unsigned_right_shift(value, 7) as i32;
    }
}

fn write_string(data: String) -> Vec<u8> {
    let mut buf = Vec::new();
    let buf_data = data.bytes();
    buf.extend(write_int(buf_data.len() as i32));
    buf.extend(buf_data);
    buf
}
static mut NEW_LEVEL_COUNTER: i32 = 1;

fn detect_new_level(stream: &TcpStream, z: f64) {
    unsafe {
        // println!("\nIS WORK\n {}",  (NEW_LEVEL_COUNTER * 16) + 3);
        if z > (3 + (NEW_LEVEL_COUNTER * 16)) as f64 {
            NEW_LEVEL_COUNTER += 1;
            write_bytes(stream, create_level_2(0, NEW_LEVEL_COUNTER));
        }
    }
}

static mut OLD_LEVEL_COUNTER: i32 = 1;

fn detect_old_level(stream: &TcpStream, z: f64) {
    unsafe {
        // println!("\nIS WORK\n {}",  (NEW_LEVEL_COUNTER * 16) + 3);
        if z > (12 + (OLD_LEVEL_COUNTER * 16)) as f64 {
            // println!("\n#####UNLOAD, Chunk {}\n", OLD_LEVEL_COUNTER -2);
            OLD_LEVEL_COUNTER += 1;
            write_bytes(stream, create_level_empty(0, OLD_LEVEL_COUNTER -2));
            write_bytes(stream, create_unload_chunk(0, OLD_LEVEL_COUNTER -2));
        }
    }
}


fn skip_lisen(mut stream: &TcpStream) {
    let size = read_int(stream);
    let id = read_int(stream) as u8;
    // println!("SKIP size: {} with id: {} OR hexId: 0x{:X}", size, id, id);
    match id {

        20 => {
            let x = read_double(stream);
            let y = read_double(stream);
            let z =  read_double(stream);
            let on_ground = {if read_byte(stream) == 1 {true} else {false}};
            // print!(" Position\tX: {}\tFeet Y: {},\tZ: {}\tOn Ground: {}\n", x, y, z, on_ground);


            if on_ground || true {
                detect_new_level(stream, z);
                detect_old_level(stream, z);
            }

            if y < -100.0 {
                write_bytes(stream, set_health(0.0, 20, 0.0));
                thread::sleep_ms(1000);
                unsafe { if OLD_LEVEL_COUNTER > MAX_SCORE {MAX_SCORE = OLD_LEVEL_COUNTER}};
                write_bytes(stream, create_message("Ти програв, твій рівень: ".to_owned() + unsafe {&OLD_LEVEL_COUNTER.to_string()}));
                stream.flush().unwrap();
                panic!();
            }


            let mut chunk_x = x.round() as i32 / 16;
            let mut chunk_z = (z.round() as i32 +5) / 16;
            if x < 0.0 {
                chunk_x -= 1;
            }
            if z < 0.0 {
                chunk_z -= 1;
            }

            // println!("Chunk X: {} Z: {}", chunk_x, chunk_z);
            // if chunk_z != 0 {
            //     write_bytes(stream, create_level_2(chunk_x, chunk_z));
            // }
            // write_bytes(stream, create_level((x.round() as i32 / 16)  +1, (z.round() as i32 / 16) +1));
            // write_bytes(stream, create_level((x.round() as i32 / 16) -1, (z.round() as i32 / 16) -1));
        }
        
        // 5 => {
        //     println!("Player send to chat: {}", read_string(stream));
        //     loop {
        //         let _read = read_byte(stream);
        //         println!("SKIP {}", _read);
        //     }
        // }
        
        
        _ => {
            for _ in 1..size {
                let _read = read_byte(stream);
                // println!("SKIP {}", read);
            }
        }
    }
}

fn network_handle(mut stream: &TcpStream, id: i32, size: i32) {
    // println!(
    //     "{:?}, Id packege: {}, size: {}",
    //     thread::current().id(),
    //     id,
    //     size
    // );

    // println!();

    match id {
        0 => {
            let protocol_version = read_int(&stream);
            let adderss = read_string(&stream);
            let port = read_unsigned_short(&stream);
            let status = read_int(stream);
            // println!(
            //     "CONNECT INFO: {} {} {} {}",
            //     protocol_version, adderss, port, status
            // );
            if status == 1 {
                skip_lisen(stream);
                status_response(&stream, protocol_version);
                skip_lisen(stream);
                stream
                    .write(b"\t\x01\x00\x00\x00\x00\x00\xd4\xedQ")
                    .unwrap();
            } else {
                skip_lisen(stream);
                // write_bytes(stream, b"\x03\x00".to_vec());

                // println!("{}", String::from_utf8_lossy(b"\x02\xaa\x31\xd6\xfb\xe4\x69\x3a\xe4\x9b\x7f\x97\xc6\x3c\x75\x4c\x88\x0b\x44\x6d\x79\x74\x72\x6f\x46\x72\x61\x6d\x65\x00"));
                write_bytes(stream, b"\x02\xaa\x31\xd6\xfb\xe4\x69\x3a\xe4\x9b\x7f\x97\xc6\x3c\x75\x4c\x88\x0b\x44\x6d\x79\x74\x72\x6f\x46\x72\x61\x6d\x65\x00".to_vec());
                write_bytes(stream, consts::LOGIN.to_vec());

                // write_bytes(stream, b"\x16\x0f\x6d\x69\x6e\x65\x63\x72\x61\x66\x74\x3a\x62\x72\x61\x6e\x64\x05\x50\x61\x70\x65\x72".to_vec());
                // write_bytes(stream, b"\x0b\x02\x00".to_vec());
                // write_bytes(stream, b"\x31\x0d\x3d\x4c\xcc\xcd\x3d\xcc\xcc\xcd".to_vec());

                write_bytes(stream, b"\x4a\x01".to_vec());
                write_bytes(stream, b"\x39\x3f\xd7\xed\xf7\xe8\x0b\x69\x30\x40\x00\x00\x00\x00\x00\x00\x00\x3f\xe8\x2f\x52\x41\x4d\x84\x7c\x41\x17\x33\xe6\x40\xf4\xc6\xfc\x00\x01\x00".to_vec());
                write_bytes(stream, create_level_1(0, 0));
                write_bytes(stream, create_level_2(0, 1));
                // write_bytes(stream, b"".to_vec());
                // write_bytes(stream, b"\x39\xc0\x66\x10\x00\x00\x00\x00\x00\x40\x10\x00\x00\x00\x00\x00\x00\x40\x86\xd4\x00\x00\x00\x00\x00\xc2\xb2\xd0\x1d\x41\x17\x33\x35\x00\x01\x00".to_vec());
                // write_bytes(stream, b"\x62\xd2\x02\x7b\x22\x63\x6f\x6c\x6f\x72\x22\x3a\x22\x79\x65\x6c\x6c\x6f\x77\x22\x2c\x22\x74\x72\x61\x6e\x73\x6c\x61\x74\x65\x22\x3a\x22\x6d\x75\x6c\x74\x69\x70\x6c\x61\x79\x65\x72\x2e\x70\x6c\x61\x79\x65\x72\x2e\x6a\x6f\x69\x6e\x65\x64\x22\x2c\x22\x77\x69\x74\x68\x22\x3a\x5b\x7b\x22\x69\x6e\x73\x65\x72\x74\x69\x6f\x6e\x22\x3a\x22\x44\x6d\x79\x74\x72\x6f\x46\x72\x61\x6d\x65\x22\x2c\x22\x63\x6c\x69\x63\x6b\x45\x76\x65\x6e\x74\x22\x3a\x7b\x22\x61\x63\x74\x69\x6f\x6e\x22\x3a\x22\x73\x75\x67\x67\x65\x73\x74\x5f\x63\x6f\x6d\x6d\x61\x6e\x64\x22\x2c\x22\x76\x61\x6c\x75\x65\x22\x3a\x22\x2f\x74\x65\x6c\x6c\x20\x44\x6d\x79\x74\x72\x6f\x46\x72\x61\x6d\x65\x20\x22\x7d\x2c\x22\x68\x6f\x76\x65\x72\x45\x76\x65\x6e\x74\x22\x3a\x7b\x22\x61\x63\x74\x69\x6f\x6e\x22\x3a\x22\x73\x68\x6f\x77\x5f\x65\x6e\x74\x69\x74\x79\x22\x2c\x22\x63\x6f\x6e\x74\x65\x6e\x74\x73\x22\x3a\x7b\x22\x74\x79\x70\x65\x22\x3a\x22\x6d\x69\x6e\x65\x63\x72\x61\x66\x74\x3a\x70\x6c\x61\x79\x65\x72\x22\x2c\x22\x69\x64\x22\x3a\x22\x61\x61\x33\x31\x64\x36\x66\x62\x2d\x65\x34\x36\x39\x2d\x33\x61\x65\x34\x2d\x39\x62\x37\x66\x2d\x39\x37\x63\x36\x33\x63\x37\x35\x34\x63\x38\x38\x22\x2c\x22\x6e\x61\x6d\x65\x22\x3a\x7b\x22\x74\x65\x78\x74\x22\x3a\x22\x44\x6d\x79\x74\x72\x6f\x46\x72\x61\x6d\x65\x22\x7d\x7d\x7d\x2c\x22\x74\x65\x78\x74\x22\x3a\x22\x44\x6d\x79\x74\x72\x6f\x46\x72\x61\x6d\x65\x22\x7d\x5d\x7d\x00".to_vec());
                // write_bytes(stream, b"\x5c\x00\x00\x00\x00\x00\x00\x50\xcf\x00\x00\x00\x00\x00\x00\x50\xcf".to_vec());
                // write_bytes(stream, b"\x4d\xff\xff\xd2\x80\x00\x2d\x40\x04\x00\x00\x00\x00".to_vec());
                // write_bytes(stream, b"".to_vec());
               

                // stream.write(b"\x02\x66\x51\xd5\xac\x5a\x3a\xa9\x45\x43\x91\x5d\xb2\xaf\x22\x21\x06\x1e\x04\xc0\x28\x80\x00\x00\x00\x00\x00\x40\x45\x8c\xcc\xcd\x00\x00\x00\x40\x40\xe0\x00\x00\x00\x00\x00\xe0\x00\xaf\x00\x00\x00").unwrap();
                // let mut buf = Vec::new();
                // buf.push(3);
                // buf.extend(write_int(-1));

                // write_bytes(stream, buf);
                // skip_lisen(stream);
                // const SLEEP: u32 = 8;

                // thread::spawn(move || {
                //    loop {

                //    }
                // });

                // thread::sleep_ms(4000);
                write_bytes(stream, create_game_event(3, 0)); // set survaivel mode
                
                // for x in -20..20 {
                //     println!("Spawn chunk line");
                //     for z in -29999960..-29999950 {
                //         println!("Spawn chunk");
                //     // thread::sleep_ms(100);
                        
                //     write_bytes(stream, create_level(x, z));
                // }
                // }
                write_bytes(stream, set_player_position(6.7, 2.0, 7.2));
                unsafe {
                    NEW_LEVEL_COUNTER = 1;
                    OLD_LEVEL_COUNTER = 1;
                }
                loop {
                   
                    // for i in 1..=20 {
                    //     thread::sleep_ms(SLEEP);
                    //     write_bytes(stream, set_health(f32::from(i as f32), i, 0.0));
                    // }

                    // for i in (1..=20).rev() {
                    //     thread::sleep_ms(SLEEP);
                    //     write_bytes(stream, set_health(f32::from(i as f32), i, 0.0));
                    // }

                    // thread::sleep_ms(3000);
                    // write_bytes(stream, set_health(20.0, 15, 0.0));
                    skip_lisen(stream);
                    write_bytes(stream, set_health(20.0, 20, 0.0));

                    




                    // thread::sleep_ms(SLEEP);
                    // write_bytes(stream, create_game_event(3, 1));
                    // thread::sleep_ms(SLEEP);
                    // write_bytes(stream, create_game_event(3, 0));
                    // thread::sleep_ms(SLEEP);
                    // write_bytes(stream, b"\x39\x3f\xd7\xed\xf7\xe8\x0b\x69\x30\x40\x00\x00\x00\x00\x00\x00\x00\x3f\xe8\x2f\x52\x41\x4d\x84\x7c\x41\x17\x33\xe6\x40\xf4\xc6\xfc\x00\x01\x00".to_vec());
                    // write_bytes(stream, create_game_event(11, 1));
                }

                // let mut buf = Vec::new();
                // for _ in 0..read_int(stream) {
                //     buf.push(read_byte(stream));
                // }
                // println!("{:?}", buf);
                // println!("{}", String::from_utf8_lossy(&mut buf));
            }
        }

        _ => {}
    }
}

fn create_level(x: i32, z: i32) -> Vec<u8> {
    let mut buf_level = Vec::new();
    buf_level.push(33);
    buf_level.extend(i32::from(x).to_be_bytes());
    buf_level.extend(i32::from(z).to_be_bytes());
    buf_level.extend(consts::MAP_LEVEL_WITH_LIGHT);
    buf_level
}

fn create_level_1(x: i32, z: i32) -> Vec<u8> {
    let mut buf_level = Vec::new();
    buf_level.push(33);
    buf_level.extend(i32::from(x).to_be_bytes());
    buf_level.extend(i32::from(z).to_be_bytes());
    buf_level.extend(consts::LEVEL_1);
    buf_level
}

fn create_level_2(x: i32, z: i32) -> Vec<u8> {
    let mut buf_level = Vec::new();
    buf_level.push(33);
    buf_level.extend(i32::from(x).to_be_bytes());
    buf_level.extend(i32::from(z).to_be_bytes());
    buf_level.extend(consts::LEVEL_2);
    buf_level
}

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
    buf.extend(write_string(message.to_string()));
    buf
}


static mut MAX_SCORE: i32 = 0;

fn status_response(stream: &TcpStream, protocol: i32) {
    // let mut file = File::open("config/server_status.json").unwrap();
    // let mut buffer = String::new();
    // file.read_to_string(&mut buffer).unwrap();

    let mut string = String::new();
    string.push_str(r###"{ "version": { "name": "2", "protocol": "###);
    string.push_str(&format!("{}", protocol));
    string.push_str(r###" }, "players": { "max": -7, "online": 1000, "sample": [ { "name": "DmytroFrame", "id": "4566e69f-c907-48ee-8d71-d7ba5aa00d20" } ] }, "description": { "text": "Infinity parkour! Record level: "###);
    string.push_str(unsafe {&MAX_SCORE.to_string()});
    string.push_str(r###""}, "favicon": "data:image/png;base64,<>", "previewsChat": false }"###);

    let mut buf = Vec::new();
    buf.push(0);
    buf.extend(write_string(string));
    write_bytes(stream, buf);
}

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
    buf.extend(write_int(food));
    buf.extend(f32::from(food_saturation).to_be_bytes());
    buf
}


fn set_player_position(X: f64, Y: f64, Z: f64) -> Vec<u8> {
    let mut buf = Vec::new();
    buf.push(57);
    buf.extend(X.to_be_bytes());
    buf.extend(Y.to_be_bytes());
    buf.extend(Z.to_be_bytes());
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


// fn play_login(stream: &TcpStream) {
//     let mut buf: Vec<u8> = Vec::new();

//     buf.extend(i32::from(1).to_be_bytes());
//     buf.extend(u8::from(1).to_be_bytes());
//     buf.push(1);

// }
