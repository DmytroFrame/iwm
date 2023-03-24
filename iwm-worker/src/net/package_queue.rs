use crate::logger::Logger;
use crate::net::protocol::server::disconnect::Disconnect;

use super::protocol::package_input::{input_package_handle, InputPackage};
use super::protocol::package_output::{output_package_handle, OutputPackage};
use super::protocol::utils::package_header::PackageHeader;
// use super::protocol::utils::buffer_writer::BufferWriter;
use super::protocol::utils::stream_reader::StreamReader;
use tokio::io::{split, AsyncWriteExt, ReadHalf, WriteHalf};
use tokio::net::TcpStream;
use tokio::sync::mpsc::{channel, Receiver, Sender};

const MAX_PACKETS_IN_QUEUE: usize = 200;

pub(crate) struct PlayerStream {
    pub input: Receiver<InputPackage>,
    pub output: Sender<OutputPackage>,
}

pub(crate) async fn create_package_queue(stream: TcpStream) -> PlayerStream {
    let (stream_tx, stream_rx): (ReadHalf<TcpStream>, WriteHalf<TcpStream>) = split(stream);
    let (writer_tx, writer_rx): (Sender<OutputPackage>, Receiver<OutputPackage>) =
        channel(MAX_PACKETS_IN_QUEUE);
    let (reader_tx, reader_rx): (Sender<InputPackage>, Receiver<InputPackage>) =
        channel(MAX_PACKETS_IN_QUEUE);

    tokio::spawn(reader_loop(stream_tx, reader_tx));

    tokio::spawn(writer_loop(stream_rx, writer_rx));

    PlayerStream {
        input: reader_rx,
        output: writer_tx,
    }
}

async fn reader_loop(mut stream_tx: ReadHalf<TcpStream>, reader_tx: Sender<InputPackage>) {
    loop {
        match PackageHeader::from_steam(&mut stream_tx).await {
            Ok(header) => {
                let package =
                    input_package_handle(header.size - 1, header.id, &mut stream_tx).await;

                // Logger::new("ReaderIO").info(&format!("{:?}", package));

                reader_tx.send(package).await.unwrap();
            }

            Err(err) => {
                println!("Disconnect: {:?}", err);
                reader_tx.send(InputPackage::Disconnect).await.unwrap();
                reader_tx.closed().await;
                break;
            }
        }
    }
}

async fn writer_loop(mut stream_rx: WriteHalf<TcpStream>, mut writer_rx: Receiver<OutputPackage>) {
    loop {
        match writer_rx.recv().await {
            None => {
                println!("None on writer_loop:");
                writer_rx.close();
                break;
            }

            Some(package) => {
                match &package {
                    OutputPackage::ChunkDataAndUpdateLight(_) => {}

                    any => Logger::new("WriterIO").debug(&format!("{:?}", any)),
                }

                let buffer = output_package_handle(package).await;
                stream_rx.write(&buffer).await.unwrap();
            }
        }
    }
}
