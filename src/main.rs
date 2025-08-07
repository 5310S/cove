use std::error::Error;
use std::sync::Arc;
use tokio::io::{ReadHalf, WriteHalf};
use tokio::net::{TcpStream};
use tokio::sync::Mutex;
use tokio::task;
use tokio::time::{sleep, Duration};

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
    // Animation
    for i in (1..=35).rev() {
        println!("{}", "*".repeat(i));
        sleep(Duration::from_millis(50)).await;
    }
    println!("5310S Presents");
    sleep(Duration::from_millis(500)).await;
    for _ in 0..4 {
        println!("*");
        sleep(Duration::from_millis(100)).await;
    }
    println!("Lantern - Illuminate your path");
    println!();
    sleep(Duration::from_millis(1500)).await;
    println!("Getting things together, just a moment...");
    println!();
    sleep(Duration::from_millis(1000)).await;

    // Arc Vectors
    let connections_read: Arc<Mutex<Vec<ConnectionRead>>> = Arc::new(Mutex::new(Vec::new()));
    let connections_write: Arc<Mutex<Vec<ConnectionWrite>>> = Arc::new(Mutex::new(Vec::new()));


    // Arc Pointers
    let connections_read_listener = Arc::clone(&connections_read);
    let connections_write_listener = Arc::clone(&connections_write);
    let connections_read_caller = Arc::clone(&connections_read);
    let connections_write_caller = Arc::clone(&connections_write);


    // Tasks
    let _connection_listener = task::spawn(async move {
        if let Err(e) = networking::connections::tcp::main_listener(connections_read_listener, connections_write_listener).await {
            eprintln!("Main listener error: {}", e);
        }
    });


    let _connection_caller = task::spawn(async move {
        if let Err(e) = networking::connections::tcp::peerlist_caller(connections_read_caller, connections_write_caller).await {
            eprintln!("Peer connection error: {}", e);
        }
    });

    // graceful shutdown
    tokio::signal::ctrl_c().await?;
    Ok(())
}

