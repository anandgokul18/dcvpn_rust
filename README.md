# [Open Source] Blockchain Decentralized Lightweight VPN in Rust

DCVPN_Rust (Decentralized VPN in Rust) is an open-source initiative started by @anandgokul18 to design, develop and maintain a decentralized Virtual Private Network (VPN) solution, meticulously engineered using the Rust programming language. Chosen for its superior performance characteristics, Rust empowers this project with high-speed execution, rigorous memory safety, and seamless concurrency capabilities. 

This project's central design philosophy revolves around the creation of a lightweight, yet robust VPN solution, characterised by its ease of use and adaptability. DCVPN_RUST is underpinned by a configuration system that simplifies deployment across multiple hosts, making it an ideal choice for diverse networking environments. 

With a design that encourages direct UDP communication between selected hosts, DCVPN_RUST eliminates the need for a central server, thereby fostering a truly decentralized network. Built with Linux systems in mind, it harnesses the power of TUN devices to deliver a high-performance networking solution.

One of the cornerstones of DCVPN_RUST is its commitment to security. It employs AES-128, AES-192, or AES-256 encryption, giving users the flexibility to choose between speed and security, as per their requirements. Additionally, an optional HMAC-SHA256 encryption is available for enhanced data integrity and authenticity.

DCVPN_RUST is a testament to the power of open-source development. It is not only a product of rigorous engineering and meticulous design but also of the collective effort of a passionate community. As an open-source project, it welcomes contributions from developers around the world, fostering a culture of collaboration and continuous improvement.

However, it's important to note that while DCVPN_RUST is a powerful tool, it is still in its beta stage. Its use comes with a set of risks and it's recommended to use only the versions marked as "release". Despite this, we're confident in the potential of this project and are continually working towards enhancing its capabilities and stability.

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
$ git clone https://github.com/anandgokul18/dcvpn_rust
$ cd dcvpn_rust
$ cargo build --release
```

You can run DCVPN_RUST with the following commands:

If you have a config in /etc/dcvpn_rust.conf:
```
$ sudo ./target/release/dcvpn_rust
```

If you want to specify a different location for the config file (or if you need to run several instances):
```
$ sudo ./target/release/dcvpn_rust -config dcvpn_rust.conf
```

If your host is hidden behind a firewall (with UDP port forward) and DCVPN_RUST is unable to detect which "remote" is localhost, use the following syntax:
```
$ sudo ./target/release/dcvpn_rust -local berlin -config dcvpn_rust.conf
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
LocIP = 192.168.3.3
route = 192.168.12.0/24

```

The configuration parameters are defined as follows:

- port is the UDP port for communication.
- encryption can be set to aescbc for AES-CBC, aescbchmac for AES-CBC+HMAC-SHA245, or none for no encryption.
- For aescbc, mainkey/altkey is the hexadecimal form of a 16, 24, or 32 bytes key (for AES-128, AES-192, or AES-256).
- For aescbchmac, mainkey/altkey is 32 bytes longer.
- For none, mainkey/altkey is just ignored.
- Number of remotes is virtually unlimited, each taking about 256 bytes in memory.

# Configuration Reload

The configuration is reloaded on a HUP signal. In the case of an invalid configuration, a log message will appear, and the previous configuration will be used.

Please note: the listening UDP socket is not currently reopened, so a restart is required when changing the port.

# Online Key Change

The altkey configuration option allows you to specify an alternative encryption key that will be used if decryption with the primary one fails. This allows the following algorithm to change keys without taking the link offline:

- In normal state, only mainkey is set (setting altkey is more CPU-consuming).
- Set altkey to the new key on all hosts and send a HUP signal.
- Swap altkey and mainkey on all hosts and send a HUP signal.
- Remove altkey (with the old key) from the configurations on all hosts and send a HUP signal again.
- The system is now running with the new key :)

# Roadmap

- 100% unit test coverage.
- Support for additional platforms.
- Improvements in efficiency and performance.
- If there's something more you need, please let us know!

# Contributing

We welcome contributions from the open source community! Whether it's improving the code, adding new features, fixing bugs, or enhancing the documentation, your contributions are always appreciated.

If you're thinking of contributing, please follow these steps:

1. **Fork the repository** - This creates your own copy of the project where you can make your changes.

2. **Create a new branch** - This keeps your main branch clean and makes it easier to integrate your changes later.

3. **Make your changes** - Make sure to thoroughly test your changes!

4. **Submit a pull request** - Push your changes to your branch on GitHub and open a pull request against the main branch in the original repository. Please include a clear description of the changes you've made in your pull request.

All pull requests will be reviewed by the maintainers of the project. Feedback may be given and changes requested. Once everything is in order, your pull request will be merged into the main codebase.

We look forward to your contributions and to collaborating with you!

# Code of Conduct

In the interest of fostering an open and welcoming environment, we as contributors and maintainers pledge to make participation in our project and our community a harassment-free experience for everyone, regardless of age, body size, disability, ethnicity, sex characteristics, gender identity and expression, level of experience, education, socio-economic status, nationality, personal appearance, race, religion, or sexual identity and orientation.

We expect all participants in our community to abide by this code of conduct. Please report any unacceptable behavior to the project maintainers.

Thank you for helping us create a positive environment for everyone!
