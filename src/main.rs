use std::env;
use std::net::TcpStream;
use std::str::FromStr;
use std::sync::mpsc::channel;
use std::sync::Arc;
use std::time::Duration;
use threadpool::ThreadPool;


fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 3 && args.len() != 4 {
        eprintln!("Usage: {} <IP> <port> or {} <IP> <start port> <end portu>", args[0], args[0]);
        std::process::exit(1);
    }

    let ip = &args[1];
    let start_port = u16::from_str(&args[2]).expect("Port be must a number");
    let end_port = if args.len() == 4 { u16::from_str(&args[3]).expect("Port be must a number") } else { start_port };

    scan_ports(ip, start_port, end_port);
}

fn scan_ports(ip: &str, start_port: u16, end_port: u16) {
    let num_threads = 4; // You can set the thread count here
    let pool = ThreadPool::new(num_threads);
    let (sender, receiver) = channel();
    let ip = Arc::new(ip.to_string());

    for port in start_port..=end_port {
        let sender = sender.clone();
        let ip = Arc::clone(&ip);
        
        pool.execute(move || {
            let address = format!("{}:{}", ip, port);
            match TcpStream::connect_timeout(&address.parse().unwrap(), Duration::new(1, 0)) {
                Ok(_) => {
                    println!("Port Open!: {}", port);
                    sender.send(port).unwrap();
                },
                Err(_) => {}
            }
        });
    }

    pool.join(); // Wait for all threads to finish
    drop(sender); // Turn off Sender

    let open_ports: Vec<u16> = receiver.iter().collect();
    if !open_ports.is_empty() {
        println!("Open Ports: {:?}", open_ports);
    } else {
        println!("Open ports not found.");
    }
}

