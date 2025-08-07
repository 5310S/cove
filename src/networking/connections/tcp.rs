
use std::error::Error;
use tokio::io::AsyncWriteExt;
use tokio::net::{TcpListener as TokioTcpListener, TcpStream};

pub async fn main_listener() -> Result<(), Box<dyn Error>> {

    let tcp_listener = TokioTcpListener::bind("0.0.0.0:8080").await?;

    loop {
        println!("Listening for new connections...");
        let (mut stream, socket_addr) = tcp_listener.accept().await?;
        println!("Accepted connection from {}", socket_addr);

        if let Err(e) = stream.write_all(b"Received New Connection.\n").await {
            eprintln!("Failed to send to {}: {}", socket_addr, e);
        }

    }
}

pub async fn connect_to_server(addr: &str) -> Result<(), Box<dyn Error>> {
    println!("Attempting to connect to {}", addr);
    match TcpStream::connect(addr).await {
        Ok(mut stream) => {
            println!("Successfully connected to {}", addr);
            stream.write_all(b"Hello from client.\n").await?;
        }
        Err(e) => {
            eprintln!("Failed to connect to {}: {}", addr, e);
        }
    }
    Ok(())
}
