# Rust Port Scanner

## Overview
This tool is a port scanner written in Rust, providing the capability to scan individual ports as well as ranges of ports on a specified IP address. It's designed with concurrency in mind, allowing for faster scanning operations by making multiple connection attempts in parallel. This tool can be used for network diagnostics and security auditing.

## Features
- **Single Port Scanning:** Quick and easy way to check if a specific port is open on a given IP address.
- **Range Scanning:** Enables scanning of a range of ports, providing a comprehensive view of the open ports on a target machine.
- **Concurrency:** Utilizes Rust's powerful concurrency model to perform multiple scans simultaneously, significantly reducing total scan time.
- **Connection Timeout:** Each connection attempt is bounded by a timeout, ensuring the scanner does not hang indefinitely on certain ports.

## Requirements
- Rust Programming Language
- Cargo (Rust's package manager)

## Installation
Clone the repository and build the project:
```bash
git clone https://github.com/ahmetskulekci/rs-scan.git
cd rust-port-scanner

cargo build --release
```
The compiled binary will be located at "target/release/rs-scan".

##Usage
After compiling the project, you can run the port scanner using the following commands:

To scan a single port:
```bash
./target/release/rs-scan IP_ADDRESS PORT
```
To scan a range of ports:
```bash
./target/release/rust_port_scanner IP_ADDRESS START_PORT END_PORT
```

##Examples
Scan port 80 on IP address 192.168.1.1:
```bash
./target/release/rust_port_scanner 192.168.1.1 80
```

Scan ports 1 through 100 on IP address 192.168.1.1:
```bash
./target/release/rust_port_scanner 192.168.1.1 1 100
```