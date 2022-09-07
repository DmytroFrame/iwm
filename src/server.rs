use crate::{network::utils::read, network_handle, status::online};
use std::{
    io::{Read, Result, Write},
    net::{SocketAddr, TcpListener, TcpStream},
    thread,
};

pub fn start(port: u16) {
    let listener = TcpListener::bind(SocketAddr::from(([127, 0, 0, 1], port))).unwrap();
    for stream in listener.incoming() {
        thread::spawn(move || {
            online::plus();
            handle_connection(stream.expect("хуйня в stream")).unwrap();
            online::minus();
            println!("END {:?}", thread::current().id());
        });
    }
}

fn handle_connection(mut stream: TcpStream) -> Result<()> {
    let size = read::int(&stream);
    let id = read::int(&stream);

    match id {
        0x00 => network_handle(stream, id, size)?,
        0x45 => handle_http_request(stream)?,
        _ => stream.flush()?,
    }
    Ok(())
}

fn handle_http_request(mut stream: TcpStream) -> Result<()> {
    stream.read(&mut [0; 900])?; //skip data from browser

    let content = "{ online: ".to_string() + &(online::get() - 1).to_string() + " }";
    let response = format!(
        "HTTP/1.1 200 OK\r\nContent-Length: {}\r\nContent-Type: application/json; charset=utf-8\r\nAccess-Control-Allow-Origin: *\r\nX-Powered-By: iWM Server\r\n\r\n{}",
        content.len(),
        content
    );
    stream.write(response.as_bytes())?;
    stream.flush()?;
    Ok(())
}
