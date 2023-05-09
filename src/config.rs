use std::net::{IpAddr, SocketAddr};
use serde::Deserialize;
use std::fs;
use std::path::Path;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub bind_address: SocketAddr,
    pub tun_device_name: String,
    pub tun_ip: IpAddr,
    pub tun_netmask: u8,
    pub encryption_key: Vec<u8>,
    pub peers: Vec<Peer>,
}

#[derive(Debug, Deserialize)]
pub struct Peer {
    pub address: SocketAddr,
}

impl Config {
    pub fn from_file(file: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let path = Path::new(file);
        let contents = fs::read_to_string(path)?;
        let config: Config = toml::from_str(&contents)?;
        Ok(config)
    }
}
