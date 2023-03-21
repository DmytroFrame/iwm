

fn handle_client(conn: Result<TcpStream, Error>) {
    // let log = Logger::new("ClientIO");
    // let mut stream = conn.unwrap();

    // let mut buffer: [u8; 2] = [0; 2];
    // stream.read(&mut buffer).unwrap();

    // if buffer[1] != 0 {
    //     return;
    // }

    // let handshaking = server::handshaking::Handshaking::from(&mut stream);

    log.debug(&format!("{:?}", handshaking));

    stream.read(&mut [0; 256]).unwrap();

    stream
        .write(&[
            0x1e, 0x02, 0xaa, 0x31, 0xd6, 0xfb, 0xe4, 0x69, 0x3a, 0xe4, 0x9b, 0x7f, 0x97, 0xc6,
            0x3c, 0x75, 0x4c, 0x88, 0x0b, 0x44, 0x6d, 0x79, 0x74, 0x72, 0x6f, 0x46, 0x72, 0x61,
            0x6d, 0x65, 0x00,
        ])
        .unwrap();

    stream.write(mock::LOGIN).unwrap();

    // std::thread::sleep_ms(5000);

    let mut writer = PackageWriter::new();
    writer.bytes(&[
        0x39, 0x40, 0x86, 0xEE, 0xA5, 0x6F, 0xF3, 0xA0, 0x86, 0x40, 0x51, 0x74, 0x9A, 0x32, 0x9B,
        0xEE, 0x7D, 0x40, 0x87, 0x9E, 0x79, 0xBF, 0xA5, 0xC9, 0x04, 0x41, 0x51, 0x20, 0xE0, 0x41,
        0x33, 0xFE, 0x9E, 0x00, 0x01, 0x00,
    ]);
    stream.write(&writer.build()).unwrap();

    let mut writer = PackageWriter::new();
    writer.bytes(&[0x4B, 0x2D, 0x2F]);
    stream.write(&writer.build()).unwrap();

    for pat in fs::read_dir(r"C:\Projects\iwm\node-pkg-parser\chunks\bin").unwrap() {
        let form = format!("{}", pat.unwrap().path().display());

        let mut file = fs::File::open(form).unwrap();
        let mut buf: Vec<u8> = vec![];
        file.read_to_end(&mut buf).unwrap();

        let mut writer = PackageWriter::new();
        writer.bytes(&buf);
        stream.write(&writer.build()).unwrap();
    }

    let mut player = Player {
        on_ground: false,
        position: PlayerPosition {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        },
        rotation: PlayerRotation { x: 0.0, y: 0.0 },
    };

    loop {
        let mut writer = PackageWriter::new();
        writer.bytes(&[0x20, 0x00, 0x00, 0x00, 0x00, 0x08, 0xA2, 0x8E, 0x07]);
        stream.write(&writer.build()).unwrap();
        // std::thread::sleep_ms(2000);

        let mut reader = PackageReader::new(&mut stream);
        let size = reader.var_int();
        let id = reader.var_int();

        if size == 0 {
            log.info(&format!("Player disconect  {:?}", player));
            return;
        }

        match id {
            0x14 => {
                let res = server::set_player_position::SetPlayerPosition::from(&mut stream);
                player.position = PlayerPosition {
                    x: res.x,
                    y: res.y,
                    z: res.z,
                };
                player.on_ground = res.on_ground;
            }
            0x15 => {
                let res =
                    server::set_player_position_and_rotation::SetPlayerPositionAndRotation::from(
                        &mut stream,
                    );
                player.position = PlayerPosition {
                    x: res.x,
                    y: res.y,
                    z: res.z,
                };
                player.rotation = PlayerRotation {
                    x: res.yaw,
                    y: res.pitch,
                };
                player.on_ground = res.on_ground;
            }

            0x16 => {
                let res = server::set_player_rotation::SetPlayerRotation::from(&mut stream);

                log.debug(&format!("{:?}", res));
                player.rotation = PlayerRotation { x: res.x, y: res.y };
                player.on_ground = res.on_ground;
            }

            18 => {
                let mut buf = [0; 1024];
                stream.read(&mut buf).unwrap();
            }

            // 4 => {
            //     let mut buf = [0; 1024];
            //     let count = stream.read(&mut buf).unwrap();
            //     log.info(&format!("PkgId: {id}, GET: {:?}", &buf[0..count]));
            //      log.debug(&format!("{:#?}", player));
            // }

            _ => {
                let mut buf = [0; 1024];
                let count = stream.read(&mut buf).unwrap();
                log.info(&format!("Size: {size}, PkgId: {id}, GET: {:?}", &buf[0..count]));
                // log.debug(&format!("{:#?}", player));
            }
        }

        // std::thread::sleep_ms(100);
        // log.debug(&format!("{:#?}", player));

        // println!("GET: {:?}", &buf[0..count]);
    }
}

#[derive(Debug)]
struct PlayerPosition {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

#[derive(Debug)]
struct PlayerRotation {
    pub x: f32,
    pub y: f32,
}

#[derive(Debug)]
struct Player {
    pub position: PlayerPosition,
    pub rotation: PlayerRotation,
    pub on_ground: bool,
}

// fn handshaking_handler(stream: &mut TcpStream) {
//     let handshaking = server::handshaking::Handshaking::from(stream);
//     if handshaking.next_state == server::handshaking::HandshakingNextState::Status {

//     }

// }
