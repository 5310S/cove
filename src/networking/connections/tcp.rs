use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::error::Error;
use std::sync::Arc;
use tokio::io::AsyncWriteExt;
use tokio::io::{self};
use tokio::net::{TcpListener as TokioTcpListener, TcpStream};
use tokio::sync::Mutex;

use crate::ConnectionRead;
use crate::ConnectionWrite;

#[derive(Serialize, Deserialize, Debug)]
struct Peer {
    host: String,
    port: u16,
}

#[derive(Serialize, Deserialize, Debug)]
struct PeerList {
    peers: Vec<Peer>,
}

pub async fn peerlist_caller(
    connections_read_caller: Arc<Mutex<Vec<ConnectionRead>>>,
    connections_write_caller: Arc<Mutex<Vec<ConnectionWrite>>>,
) -> Result<(), Box<dyn Error>> {
    let my_ip = get_ip().await?;
    let peer_list_json = get_peer_list().await?;
    let peer_list: PeerList = serde_json::from_str(&peer_list_json)?;

    for peer in peer_list.peers {
        let addr = format!("{}:{}", peer.host, peer.port);
        if peer.host == my_ip {
            println!("This is my IP ({}), skipping...", peer.host);
            continue;
        }
        println!("Attempting to connect to {}", addr);

        match TcpStream::connect(&addr).await {
            Ok(stream) => {
                println!("Successfully connected to {}", addr);
                let (reader, mut writer) = io::split(stream);
                if let Err(e) = writer.write_all(b"Successful Call.\n").await {
                    eprintln!("Failed to send to {}: {}", addr, e);
                }
                println!("Sent message to {}", addr);
                connections_write_caller.lock().await.push(ConnectionWrite {
                    addr: addr.to_string(),
                    writer: Arc::new(Mutex::new(writer)),
                });
                connections_read_caller.lock().await.push(ConnectionRead {
                    addr: addr.to_string(),
                    reader: Arc::new(Mutex::new(reader)),
                });
            }
            Err(e) => {
                eprintln!("Failed to connect to {}: {}", addr, e);
            }
        }
    }

    Ok(())
}

pub async fn main_listener(
    connections_read_listener: Arc<Mutex<Vec<ConnectionRead>>>,
    connections_write_listener: Arc<Mutex<Vec<ConnectionWrite>>>,
) -> Result<(), Box<dyn Error>> {
    let tcp_listener = TokioTcpListener::bind("0.0.0.0:8080").await?;

    loop {
        println!("Listening for new connections...");
        let (stream, socket_addr) = tcp_listener.accept().await?;
        let (reader, mut writer) = tokio::io::split(stream);
        println!("Accepted connection from {}", socket_addr);
        if let Err(e) = writer.write_all(b"Received New Connection.\n").await {
            eprintln!("Failed to send to {}: {}", socket_addr, e);
        }
        println!("Pushing Connection to pool..");
        connections_write_listener
            .lock()
            .await
            .push(ConnectionWrite {
                addr: socket_addr.to_string(),
                writer: Arc::new(Mutex::new(writer)),
            });
        connections_read_listener.lock().await.push(ConnectionRead {
            addr: socket_addr.to_string(),
            reader: Arc::new(Mutex::new(reader)),
        });
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

async fn get_peer_list() -> Result<String, reqwest::Error> {
    println!("Downloading the peer list....");
    let peer_list = reqwest::get("https://imaginative-donut-51b03a.netlify.app/peer_list.txt")
        .await?
        .text()
        .await?;
    println!("Download Complete!\n");

    Ok(peer_list)
}

async fn get_ip() -> Result<String, reqwest::Error> {
    let ip = reqwest::get("https://api.ipify.org").await?.text().await?;
    Ok(ip)
}
