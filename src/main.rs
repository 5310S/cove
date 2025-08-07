use std::env;
use std::error::Error;
use std::sync::Arc;
use tokio::io::{ReadHalf, WriteHalf};
use tokio::net::TcpStream;
use tokio::sync::Mutex;

// Mods
mod networking {
    pub mod connections {
        pub mod tcp;
    }
}

#[derive(Debug, Clone)]
struct ConnectionRead {
    pub addr: String,
    pub reader: Arc<Mutex<ReadHalf<TcpStream>>>,
}

#[derive(Debug, Clone)]
pub struct ConnectionWrite {
    pub addr: String,
    pub writer: Arc<Mutex<WriteHalf<TcpStream>>>,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: {} [server|client] [addr]", args[0]);
        return Ok(());
    }

    match args[1].as_str() {
        "server" => {
            let connections_read: Arc<Mutex<Vec<ConnectionRead>>> =
                Arc::new(Mutex::new(Vec::new()));
            let connections_write: Arc<Mutex<Vec<ConnectionWrite>>> =
                Arc::new(Mutex::new(Vec::new()));

            networking::connections::tcp::main_listener(connections_read, connections_write)
                .await?;
        }
        "client" => {
            let addr = args
                .get(2)
                .cloned()
                .unwrap_or_else(|| "127.0.0.1:8080".to_string());
            networking::connections::tcp::connect_to_server(&addr).await?;
        }
        _ => {
            eprintln!("Invalid mode. Use 'server' or 'client'.");
        }
    }

    Ok(())
}
