use std::io::{Read, Write};
use std::net::{SocketAddr, TcpListener};
use std::str::FromStr;
use std::thread;
use std::time::Duration;


pub fn start(addr: &str, size: usize, timeout: u64) {
    let addr = SocketAddr::from_str(addr).expect("Invalid address!");
    let socket = TcpListener::bind(addr).expect("Bind address failed!");
    println!("TCP Server is listening on {}", addr);
    loop {
        let (mut client, client_addr) = socket.accept().expect("Incoming error!");
        println!("new client: {}", client_addr);
        thread::spawn(move || {
            let mut buf = vec![0; size];
            let bytes = client.read(&mut buf).expect("Receive failed!");
            println!("{} bytes received from {} through single package, remain {} bytes", bytes, client_addr, size - bytes);

            client.set_write_timeout(Some(Duration::from_secs(timeout))).expect("set write timeout failed!");
            let bytes = client.write(&buf).expect("Send failed!");
            println!("{} bytes sent to {} through single package, remain {} bytes", bytes, client_addr, size - bytes);
        });
    }
}