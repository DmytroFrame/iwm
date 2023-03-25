// fn read_chunk() {
//     let _ = get_server_path().unwrap();

//     let mut file = File::open(
//         r#"C:\Projects\iwm\iwm-java-server-generator\java-server\world\region\r.9.9.mca"#,
//     )
//     .unwrap();

//     let chunk_x = 300;
//     let chunk_z = 300;

//     let sector_offset = 2 * ((chunk_x & 31) + (chunk_z & 31) * 32);
//     println!("{:?}", sector_offset);
//     let mut sector_buffer = [0u8; 4];
//     file.seek(SeekFrom::Start(sector_offset as u64 * 4))
//         .unwrap();
//     file.read_exact(&mut sector_buffer).unwrap();
//     println!("{:?}", sector_buffer);

//     let sector_number = i32::from_be_bytes(sector_buffer) >> 8;
//     let sector_count = i32::from_be_bytes(sector_buffer) & 0xFF;
//     println!("{:?}", sector_number);
//     println!("{:?}", sector_count);

//     let offset = sector_number as u64 * 4096;
//     let length = sector_count as u64 * 4096;
//     println!("offset {:?}", offset);
//     println!("length {:?}", length);

//     let mut buffer = vec![0; length.try_into().unwrap()];
//     file.seek(SeekFrom::Start(offset)).unwrap();
//     file.read(&mut buffer).unwrap();
//     buffer.truncate(length.try_into().unwrap());

//     // println!("buffer {:?}", buffer );

//     let d = decode_reader(buffer);
//     println!("SIZE {}", &d.len());
//     // println!("{:?}", String::from_utf8_lossy(&d));

//     // // let compressed_data: Vec<u8> = vec![/* ваші стиснені дані */];
//     // let mut cursor = Cursor::new(buffer.c);
//     // let mut decoder = ZlibDecoder::new(&mut cursor);
//     // let mut decompressed_data = Vec::new();
//     // decoder.read_to_end(&mut decompressed_data).unwrap();

//     let mut wf = File::create("map2.dat").unwrap();
//     wf.write_all(&d).unwrap()
// }

// fn decode_reader(bytes: Vec<u8>) -> Vec<u8> {
//     let mut z = ZlibDecoder::new(&bytes[5..]);
//     let mut s: Vec<u8> = Vec::new();
//     z.read_to_end(&mut s).unwrap();
//     return s;

//     // Ok(s)
// }

use std::path::PathBuf;
use std::process::Stdio;

// use tokio::io::AsyncWriteExt;

use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::process::{Child, Command};
use tokio::sync::Mutex;
use tokio::time::{sleep, Duration};

lazy_static! {
    static ref LOG: Mutex<String> = Mutex::new(String::new());
    static ref PROCES_STDIN: Mutex<Option<Child>> = Mutex::new(None);
}

const MAX_LOG_LEN: usize = 500;

pub async fn java_server() {
    let path = get_server_path().unwrap();

    let mut java_process = Command::new("java")
        .current_dir(path)
        .arg("-Xmx2G")
        .arg("-Xms1G")
        .arg("-jar")
        .arg("server.jar")
        // .arg("nogui")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .unwrap();

    // let stdin = java_process.stdin.take().unwrap();
    let stdout = java_process.stdout.take().unwrap();
    let mut reader = BufReader::new(stdout);

    tokio::spawn(async move {
        loop {
            let mut line = String::new();
            reader.read_line(&mut line).await.unwrap();
            push_log(line.clone()).await;
            print!("[JavaServer] {}", line);
        }
    });

    wait_on_line("INFO]: Done (").await;
    println!("Java Server is start");

    // remove_folder_items(Path::new(format!()))

    // stdin.write_all_buf(&mut format!("gen-chunk world 200 200").as_bytes()).await.unwrap();
    // stdin.flush().await.unwrap();

    // stdin.write_all(b"Hello, world!\n").await.unwrap();
    // writeln!(stdin, "gen-chunk world 200 200");

    {
        let mut global_stdin = PROCES_STDIN.lock().await;
        *global_stdin = Some(java_process);
    }

    sleep(Duration::from_secs(1)).await;

    get_chunk("world".to_string(), 2000, 200).await;
    sleep(Duration::from_secs(1)).await;
    get_chunk("world".to_string(), 3000, 200).await;

    println!("keke");

    // std::thread::sleep_ms(99999);
}

pub async fn get_chunk(world: String, x: i32, z: i32) {
    {
        if let Some(java_process) = &mut *PROCES_STDIN.lock().await {
            let mut stdin = java_process.stdin.take().unwrap();

            stdin
                .write_all_buf(&mut format!("gen-chunk {world} {x} {z}").as_bytes())
                .await
                .unwrap();
            stdin.flush().await.unwrap();
        }
    }

    wait_on_line(&format!(" Create chunk on x:{x} z:{z}")).await;
}

fn get_server_path() -> Result<PathBuf, std::io::Error> {
    let current_path = std::env::current_dir()?.to_string_lossy().to_string();

    for entry in std::fs::read_dir("")? {
        let entry = entry?.path().to_string_lossy().to_string();
        if entry == "iwm-java-server-generator" {
            let path = PathBuf::from(format!(
                "{current_path}\\{}",
                "iwm-java-server-generator\\java-server"
            ));
            return Ok(path);
        }
        if entry == "java-server" {
            let path = PathBuf::from(format!("{current_path}\\{}", "ava-server"));
            return Ok(path);
        }
    }
    Err(std::io::Error::new(
        std::io::ErrorKind::NotFound,
        "Java server folder not found",
    ))
}

async fn wait_on_line(payload: &str) {
    loop {
        sleep(Duration::from_millis(1)).await;
        let log_line = LOG.lock().await;
        if log_line.find(payload) != None {
            return;
        }
    }
}

async fn push_log(line: String) {
    {
        let mut log = LOG.lock().await;
        *log = add_strings(log.clone(), line, MAX_LOG_LEN);
    }
}

fn add_strings(string1: String, string2: String, n: usize) -> String {
    let length1 = string1.len();
    let length2 = string2.len();
    let total_length = length1 + length2;

    if total_length <= n {
        return string1 + &string2;
    }

    let k = total_length - n;
    let mut new_string = string1[k..].to_string();

    new_string += &string2;

    new_string
}

// async fn remove_folder_items(path: &str) -> std::io::Result<()> {
//     let dir = fs::read_dir(path).await?;

//     for entry in dir {
//         let entry = entry?;

//         if entry.path().is_dir() {
//             remove_folder_items(entry.path().to_str().unwrap()).await?;
//             fs::remove_dir(entry.path()).await?;
//         } else {
//             fs::remove_file(entry.path()).await?;
//         }
//     }

//     Ok(())
// }
