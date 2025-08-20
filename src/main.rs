use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use anyhow::{Context, Result};
use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "rust-vpn")]
#[command(about = "A simple VPN implementation in Rust")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Server {
        #[arg(short, long, default_value = "0.0.0.0:8080")]
        bind: String,

        #[arg(short, long)]
        key: Option<String>,
    },
    Client {
        #[arg(short, long)]
        server: String,

        #[arg(short, long)]
        key: Option<String>,
    },
}

struct VpnServer {
    message: String
}

struct VpnClient {
    server_addr: SocketAddr,
}

impl VpnServer {
    fn new(message: &str) -> Self {
        VpnServer {
            message: message.to_string(),
        }
    }

    fn run(&self) {
        println!("VPN Server running with message: {}", self.message);
    }
}

impl VpnClient {
    fn new(server_addr: SocketAddr) -> Self {
        VpnClient { server_addr }
    }

    fn connect(&self) {
        println!("Connecting to VPN server at");
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Server { bind, key } => {
            println!("Starting server on {}", bind);
            let server = VpnServer::new("Hello from VPN Server");
            server.run();
        }
        Commands::Client { server, key: _ } => {
            let server_addr: SocketAddr = server.parse()
                .context("Invalid server address")?;
            let mut client = VpnClient::new(server_addr);
            client.connect();
        }
    }

    Ok(())
}
