use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::{TcpListener, TcpStream},
};

const SERVER_ADDRESS: &str = "127.0.0.1:8000";

#[tokio::main]
async fn main() {
    // // * Read arguments from cli
    // let delay = args()
    //     .nth(1)
    //     .unwrap_or_default()
    //     .parse::<u64>()
    //     .expect("[ERROR]: Error on retrieve the delay time(ms)");

    // println!("Delay: {}", delay);

    // * Connection
    println!("Connecting to {}", SERVER_ADDRESS);

    // * Bind
    let tcp_listener = TcpListener::bind(SERVER_ADDRESS)
        .await
        .expect("[ERROR]: Error to connect on server");

    // * Start server

    println!("Server ONE listening: {}", SERVER_ADDRESS);

    loop {
        let (stream, _socket) = tcp_listener
            .accept()
            .await
            .expect("[ERROR]: Erro to retrieve stream");

        tokio::spawn(async {
            handle_connection(stream).await;
        });
    }
}

// * Read and write from stream
// * Inverse of the client
async fn handle_connection(mut stream: TcpStream) {
    // * Read the buffer
    let mut buffer = [0u8; 1024];
    let buffer_length = stream
        .read(&mut buffer)
        .await
        .expect("[ERROR]: Erro on read message in buffer");

    let incoming_message = String::from_utf8_lossy(&buffer[..buffer_length]);
    println!("Received: {}", incoming_message);

    // // * Delay
    // thread::sleep(Duration::from_millis(delay));

    // * Write a message
    let _ = stream
        .write_all(incoming_message.as_bytes())
        .await
        .expect("[ERROR]: Error on write data at stream");

    println!("Sent: {}", incoming_message);
}
