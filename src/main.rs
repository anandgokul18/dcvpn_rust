use std::error::Error;
use std::net::{UdpSocket};
use std::sync::{mpsc, Arc, Mutex};
use std::thread;
use tokio::task;
use tun::Device;
use std::io::{Read, Write};

#[cfg(target_os = "macos")]
mod iface_darwin;

#[macro_use]
extern crate lazy_static;

mod config;
use crate::config::VPNState;
use crate::config::Config;

mod encryption_cbc;
use crate::encryption_cbc::{AesCbc, PacketEncrypter}

const MTU: i32 = 1300;
const BUFFERSIZE: usize = 1518;

// Assuming you have defined the VPNState struct in the config module
static CONFIG: Arc<Mutex<VPNState>> = Arc::new(Mutex::new(VPNState::new().unwrap()));

fn rcvr_thread(proto: &str, port: u16, iface: Arc<Mutex<tun::platform::Device>>) -> Result<(), Box<dyn Error>> {
    let socket = UdpSocket::bind(format!("{}:{}", proto, port))?;

    let mut encrypted = vec![0u8; BUFFERSIZE];
    let mut decrypted = vec![0u8; BUFFERSIZE];

    loop {
        let (amt, src) = socket.recv_from(&mut encrypted)?;

        let conf = CONFIG.lock().unwrap();

        // Assuming you've implemented the DecryptV4Chk function
        let (size, main_err) = conf.main.decrypt_v4_chk(&encrypted[..amt], &mut decrypted)?;

        if main_err.is_some() {
            // Handle decryption error
        }

        let n = iface.lock().unwrap().write(&decrypted[..size])?;

        if n != size {
            println!("Partial package written to local interface");
        }
    }
}

fn sndr_thread(socket: UdpSocket, iface: Arc<Mutex<tun::platform::Device>>) -> Result<(), Box<dyn Error>> {
    let mut packet = vec![0u8; BUFFERSIZE];
    let mut encrypted = vec![0u8; BUFFERSIZE];

    loop {
        let plen = iface.lock().unwrap().read(&mut packet)?;

        let conf = CONFIG.lock().unwrap();

        // Assuming you've implemented the Encrypt function
        let clen = conf.main.encrypt(&packet[..plen], &mut encrypted)?;

        for (ip, addr) in &conf.remotes {
            let tsize = socket.send_to(&encrypted[..clen], addr)?;

            if tsize != clen {
                println!("Only {} bytes of {} sent", tsize, clen);
            }
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {

    lazy_static! {
        pub static ref CONFIG: Arc<Mutex<Config>> = Arc::new(Mutex::new(Config::new()));
    }

    let local_cidr = "10.0.0.1/24";
    let iface = iface_darwin::iface_setup(local_cidr)?;

    let iface = Arc::new(Mutex::new(iface));

    let (tx, rx) = mpsc::channel();

    task::spawn(iface_darwin::routes_thread(iface.lock().unwrap().name()?.to_string(), rx));

    for _ in 0..2 {
        let iface_clone = Arc::clone(&iface);
        thread::spawn(move || rcvr_thread("0.0.0.0", 12345, iface_clone));
    }

    let socket = UdpSocket::bind("0.0.0.0:0")?;
    for _ in 0..2 {
        let iface_clone = Arc::clone(&iface);
        thread::spawn(move || sndr_thread(socket.try_clone().unwrap(), iface_clone));
    }

    Ok(())
}
