// use tokio::{
//     fs::{self, File},
//     io::{AsyncReadExt, AsyncWriteExt, Result},
// };

// const MAIN_DIR: &str = "storage";

// async fn create_folders() -> Result<()> {
//     fs::create_dir_all(format!("{MAIN_DIR}/players")).await?;
//     fs::create_dir_all(format!("{MAIN_DIR}/chunks")).await?;
//     Ok(())
// }

// pub async fn read_file(path: &String) -> Result<String> {
//     let path = format!("{MAIN_DIR}/{path}");
//     create_folders().await?;

//     match File::open(path).await {
//         Ok(mut file) => {
//             let mut string = String::new();
//             file.read_to_string(&mut string).await?;

//             Ok(string)
//         }

//         Err(_) => Ok(String::new()),
//     }
// }

// pub async fn write_file(path: &String, data: String) -> Result<()> {
//     let path = format!("{MAIN_DIR}/{path}");
//     create_folders().await?;

//     match File::open(&path).await {
//         Ok(mut file) => {
//             file.write(data.as_bytes()).await?;
//         }

//         Err(_) => {
//             let mut file = File::create(path).await?;
//             file.write(data.as_bytes()).await?;
//         }
//     };

//     Ok(())
// }
