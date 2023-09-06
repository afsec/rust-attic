// * nc 127.0.0.1 8000

use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::TcpStream,
};

const ECHO_SERVER_ADDRESS: &str = "127.0.0.1:8000";

#[tokio::main]
async fn main() {
    // * Connection
    println!("Connecting to {}", ECHO_SERVER_ADDRESS);

    if let Ok(mut stream) = TcpStream::connect(ECHO_SERVER_ADDRESS).await {
        // * Connected
        println!(
            "Connected to server -> {}:{}",
            stream.local_addr().unwrap().ip(),
            stream.local_addr().unwrap().port()
        );

        // * Write a message
        let message = "Hello World!";
        let _ = stream
            .write_all(message.as_bytes())
            .await
            .expect("[ERROR]: Error on write data at stream");
        // let _ = stream.flush().expect("[ERROR]: Error on flush the stream");

        println!("Sent: {}", message);

        // * Read the message
        let mut buffer = [0u8; 1024];
        let _buffer_length = stream
            .read(&mut buffer)
            .await
            .expect("[ERROR]: Erro on read message in buffer");

        let incoming_message = String::from_utf8_lossy(&buffer);
        println!("Received: {}", incoming_message);
    } else {
        eprintln!("Failed to to echo server {}", ECHO_SERVER_ADDRESS);
    }
}
