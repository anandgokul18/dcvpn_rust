use std::net::SocketAddr;
use std::sync::Arc;
use tokio::net::UdpSocket;
use tokio_tun::result::Result;
use tokio_tun::Tun;
use clap::{App, Arg};

mod encryption;
mod config;

#[tokio::main]
async fn main() -> Result<()> {
    let matches = App::new("Rust LCVPN")
        .version("0.1.0")
        .arg(
            Arg::new("config")
                .short('c')
                .long("config")
                .value_name("FILE")
                .about("Sets a custom config file")
                .takes_value(true),
        )
        .get_matches();

    let config_file = matches.value_of("config").unwrap_or("config.toml");
    let config = parse_config(config_file)?;

    let tun = setup_tun_device(&config)?;
    let socket = UdpSocket::bind(config.bind_address).await?;

    let peers: Vec<SocketAddr> = config.peers.into_iter().map(|p| p.address).collect();

    let encryption_engine = Arc::new(encryption::Encryption::new(config.encryption_key));

    // Implement logic for handling tun device, reading packets, and sending them to appropriate peers

    // Implement logic for receiving packets from peers, decrypting them, and writing them to the tun device

    Ok(())
}

fn parse_config(file: &str) -> Result<config::Config> {
    config::Config::from_file(file)
}

fn setup_tun_device(config: &Config) -> Result<Tun> {
    // Implement logic for setting up the tun device
}

