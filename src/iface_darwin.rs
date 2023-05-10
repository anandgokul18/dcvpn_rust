use std::collections::HashMap;
use std::error::Error;
use std::process::Command;
use std::sync::mpsc;
use tun::{Configuration, Device};
use ipnet::IpNet;
use crate::CONFIG;

const MTU: i32 = 1300;

pub fn iface_setup(local_cidr: &str) -> Result<tun::platform::Device, Box<dyn Error>> {
    let mut config = Configuration::default();
    config.address(local_cidr.parse()?);
    config.up();
    config.mtu(MTU as i32);

    let iface = tun::create(&config)?;
    println!("Interface allocated: {:?}", iface.name());
    Ok(iface)
}

pub async fn routes_thread(iface_name: String, refresh: mpsc::Receiver<bool>) {
    let mut current_routes: HashMap<IpNet, bool> = HashMap::new();
    loop {
        refresh.recv().unwrap();
        println!("Reloading routes...");

        let mut conf = CONFIG.lock().unwrap();

        let mut routes_to_del = current_routes.clone();

        for (route, _) in conf.routes.iter() {
            let route_str = route.to_string();
            if routes_to_del.contains_key(route) {
                routes_to_del.remove(route);
            } else {
                current_routes.insert(*route, true);
                println!("Adding route: {:?}", &route_str);

                if let Err(err) = Command::new("route")
                    .args(&["add", "-net", &route_str, "-interface", &iface_name])
                    .output()
                {
                    println!("Adding route {} failed: {:?}", route_str, err);
                }
            }
        }

        for (route, _) in routes_to_del.iter() {
            current_routes.remove(route);
            println!("Removing route: {:?}", route);

            if let Err(err) = Command::new("route")
                .args(&["delete", "-net", &route.to_string(), "-interface", &iface_name])
                .output()
            {
                println!("Error removing route \"{}\": {:?}", route, err);
            }
        }
    }
}

// You'll need to start the routes_thread in your main function or elsewhere with something like:
// task::spawn(routes_thread(iface_name, refresh));

