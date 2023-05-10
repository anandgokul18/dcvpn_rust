# [Open Source] Blockchain Decentralized Lightweight VPN in Rust

This project has been implemented in Rust for its efficiency, memory safety, and concurrency. DCVPN_RUST is a light and easy-to-use decentralized VPN solution.

## Features

- Lightweight and easy to use with a similar config for all hosts.
- Auto-detects local parameters, making it suitable for use with configuration management tools like Puppet.
- Uses AES-128, AES-192, or AES-256 encryption (note that AES-256 is **much slower** than AES-128 on most computers) + optional HMAC-SHA256 or NONE encryption (just copy without modification).
- Communicates via UDP directly to selected hosts (no central server required).
- Only works on macOS/ Linux (uses TUN device).
- Supports basic routing, making it suitable for connecting multiple networks.
- Multithreaded send and receive, making it scalable for high traffic.
- Better performance with a higher number of hosts due to the use of `SO_REUSEPORT`.
- Still in beta stage, use at your own risk (and please use only versions marked as "release").

![alt tag](https://raw.githubusercontent.com/kanocz/lcvpn/master/topology.png)

## Installation and Usage

First, you need to have Rust installed. You can install Rust from the official website: https://www.rust-lang.org/tools/install.

After Rust is installed, you can clone the repository and build the project:

```sh
$ git clone https://github.com/your_username/lcvpn
$ cd lcvpn
$ cargo build --release
```

You can run LCVPN with the following commands:

If you have a config in /etc/lcvpn.conf:
```
$ sudo ./target/release/lcvpn
```

If you want to specify a different location for the config file (or if you need to run several instances):
```
$ sudo ./target/release/lcvpn -config lcvpn.conf
```

If your host is hidden behind a firewall (with UDP port forward) and LCVPN is unable to detect which "remote" is localhost, use the following syntax:
```
$ sudo ./target/release/lcvpn -local berlin -config lcvpn.conf
```

# Configuration Example

A sample configuration file is provided below:
```
[main]
port = 23456
encryption = aescbc
mainkey = 4A34E352D7C32FC42F1CEB0CAA54D40E9D1EEDAF14EBCBCECA429E1B2EF72D21
altkey = 1111111117C32FC42F1CEB0CAA54D40E9D1EEDAF14EBCBCECA429E1B2EF72D21
broadcast = 192.168.3.255
netcidr = 24
recvThreads = 4
sendThreads = 4

[remote "prague"]
ExtIP = 46.234.105.229
LocIP = 192.168.3.15
route = 192.168.10.0/24
route = 192.168.15.0/24
route = 192.168.20.0/24

[remote "berlin"]
ExtIP = 103.224.182.245
LocIP = 192.168.3.8
route = 192.168.11.0/24

[remote "kiev"]
ExtIP = 95.168.211.37
LocIP = 192.168.3

```
