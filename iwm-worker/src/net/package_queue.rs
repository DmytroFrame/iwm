use std::time::Duration;

use super::protocol::package_input::{input_package_handle, InputPackage};
use super::protocol::utils::buffer_writer::BufferWriter;
use super::protocol::utils::stream_reader::StreamReader;
use tokio::io::{split, AsyncReadExt, AsyncWriteExt, ReadHalf, WriteHalf};
use tokio::net::TcpStream;
use tokio::stream;
use tokio::sync::mpsc::{channel, Receiver, Sender};

const MAX_PACKETS_IN_QUEUE: usize = 200;

pub(crate) async fn create_package_queue(
    stream: TcpStream,
) -> (Sender<u8>, Receiver<InputPackage>) {
    let (stream_tx, stream_rx): (ReadHalf<TcpStream>, WriteHalf<TcpStream>) = split(stream);
    let (writer_tx, writer_rx): (Sender<u8>, Receiver<u8>) = channel(MAX_PACKETS_IN_QUEUE);
    let (reader_tx, reader_rx): (Sender<InputPackage>, Receiver<InputPackage>) =
        channel(MAX_PACKETS_IN_QUEUE);

    tokio::spawn(reader_loop(stream_tx, reader_tx));

    tokio::spawn(writer_loop(stream_rx, writer_rx));

    (writer_tx, reader_rx)
}

async fn reader_loop(mut stream_tx: ReadHalf<TcpStream>, reader_tx: Sender<InputPackage>) {
    loop {
        let mut reader = StreamReader::new(&mut stream_tx);

        let size = reader.var_int().await;
        let id = reader.var_int().await;

        // let mut buffer: Vec<u8> = vec![0; size as usize - 1];

        // let count = stream_tx.read(&mut buffer).await.unwrap();

        // print!(
        //     "len: {}  real: {} id:{id} {:02X?}",
        //     buffer.len(),
        //     count,
        //     buffer
        // );

        let package = input_package_handle(size- 1, id, &mut stream_tx).await;

        reader_tx.send(package).await.unwrap();
    }
}

async fn writer_loop(mut stream_rx: WriteHalf<TcpStream>, mut writer_rx: Receiver<u8>) {
    loop {
        // let byte = writer_rx.recv().await.unwrap();
        // stream_rx.write_u8(byte).await.unwrap();

        // keeep alive package
        let mut writer = BufferWriter::new();
        writer.bytes(&[0x20, 0x00, 0x00, 0x00, 0x00, 0x08, 0xA2, 0x8E, 0x07]);
        stream_rx.write(&writer.build()).await.unwrap();
        tokio::time::sleep(Duration::from_secs(20)).await;
        
    }
}

// const KEEEP_ALIFE_INTERVAL: i64 = 60 * 20;
// use std::time::{SystemTime, UNIX_EPOCH};

// fn get_time_ms() -> i64 {
//     SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis() as i64
// }
