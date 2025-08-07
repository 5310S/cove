use std::env;
use std::error::Error;


// Mods
mod networking {
    pub mod connections {
        pub mod tcp;
    }
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
            networking::connections::tcp::main_listener().await?;

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
