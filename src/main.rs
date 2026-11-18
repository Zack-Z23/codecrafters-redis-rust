#![allow(unused_imports)]

use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpListener;
use tokio::net::TcpStream;

use tokio::spawn;

#[tokio::main]
async fn main() {
    // You can use print statements as follows for debugging, they'll be visible when running tests.
    println!("Logs from your program will appear here!");

    // Uncomment the code below to pass the first stage
    //
    let listener = TcpListener::bind("127.0.0.1:6379").await.unwrap();
    loop {
        let (mut stream, _addr) = listener.accept().await.unwrap();

        tokio::spawn(async move {
            let mut buf = [0; 512];

            loop {
                let bytes_read = stream.read(&mut buf).await.unwrap();
                if bytes_read == 0 {
                    break;
                }
                stream.write_all(b"+PONG\r\n").await.unwrap();
            }
        });
    }
}
