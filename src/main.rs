use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpListener;
use tokio::fs::File;
use std::error::Error;
use utils::logger;

mod utils;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let listener = TcpListener::bind("127.0.0.1:7280").await?;
    logger::info("Listening on \"127.0.0.1:7280\"");

    loop {
        let (mut socket, _) = listener.accept().await?;
        tokio::spawn(async move {
            let mut buffer = [0; 1024];
            let mut file = File::create("received_file").await.unwrap();

            loop {
                let bytes_read = socket.read(&mut buffer).await.unwrap();
                if bytes_read == 0 {
                    break;
                }
                file.write_all(&buffer[..bytes_read]).await.unwrap();
            }
        });
    }
}
