use tokio::fs::OpenOptions;
use tokio::{
    fs::create_dir_all,
    io::{AsyncReadExt, AsyncWriteExt, Result},
};

const MAIN_DIR: &str = "storage";

async fn create_folders() -> Result<()> {
    create_dir_all(format!("{MAIN_DIR}/players")).await?;
    create_dir_all(format!("{MAIN_DIR}/chunks")).await?;
    Ok(())
}

pub async fn read_file(path: &String) -> Result<String> {
    let path = format!("{MAIN_DIR}/{path}");
    create_folders().await?;

    match OpenOptions::new().read(true).open(path).await {
        Err(_) => Ok(String::new()),

        Ok(mut file) => {
            let mut buf = vec![];
            file.read_to_end(&mut buf).await.unwrap();

            Ok(String::from_utf8_lossy(&buf).to_string())
        }
    }
}

pub async fn write_file(path: &String, data: String) -> Result<()> {
    let path = format!("{MAIN_DIR}/{path}");
    create_folders().await?;

    let mut file = OpenOptions::new()
        .write(true)
        .truncate(true)
        .create(true)
        .open(path)
        .await
        .expect("Не вдалося відкрити файл");

    file.write_all(data.as_bytes())
        .await
        .expect("Не вдалося записати дані в файл");

    file.flush().await.expect("Не вдалося записати дані в файл");

    Ok(())
}
