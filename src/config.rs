use std::collections::HashMap;
use std::error::Error;
use std::net::IpAddr;
use std::sync::{Arc, atomic::{AtomicPtr, Ordering}};
use tokio::signal::unix::{signal, SignalKind};
use toml;
use serde::Deserialize;


#[derive(Deserialize)]
struct Main {
    port: u16,
    main_key: String,
    alt_key: Option<String>,
    encryption: String,
    broadcast: IpAddr,
    net_cidr: u8,
    recv_threads: Option<usize>,
    send_threads: Option<usize>,
}

#[derive(Deserialize)]
struct Remote {
    ext_ip: IpAddr,
    loc_ip: IpAddr,
    route: Vec<IpAddr>,
}

#[derive(Deserialize)]
pub struct VPNState {
    pub main: Main,
    pub remotes: HashMap<String, Remote>,
}

impl VPNState {
    pub fn new() -> Result<Self, Box<dyn Error>> {
        let contents = std::fs::read_to_string("/etc/lcvpn.conf")?;
        let state: VPNState = toml::from_str(&contents)?;
        Ok(state)
    }
}

pub struct Config {
    state: AtomicPtr<VPNState>,
}

impl Config {
    pub fn new() -> Result<Arc<Self>, Box<dyn Error>> {
        let state = VPNState::new()?;
        let config = Arc::new(Config {
            state: AtomicPtr::new(Box::into_raw(Box::new(state))),
        });
        let cloned_config = Arc::clone(&config);
        tokio::spawn(async move {
            let mut stream = signal(SignalKind::hangup()).unwrap();
            while stream.recv().await.is_some() {
                if let Ok(state) = VPNState::new() {
                    let old_state = cloned_config.state.swap(Box::into_raw(Box::new(state)), Ordering::SeqCst);
                    unsafe {
                        Box::from_raw(old_state);
                    }
                }
            }
        });
        Ok(config)
    }

    pub fn get_state(&self) -> &VPNState {
        unsafe {
            &*self.state.load(Ordering::SeqCst)
        }
    }
}

