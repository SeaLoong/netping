use std::io::{Read, Write};
use std::net::{SocketAddr, TcpStream};
use std::str::FromStr;
use std::time::Duration;


pub fn ping(addr: &str, size: usize, timeout: u64) {
    let addr = SocketAddr::from_str(addr).expect("Invalid address!");
    println!("Connect to {}", addr);
    let mut socket = TcpStream::connect(addr).expect("Connect failed!");
    println!("Connection succeed");

    socket.set_write_timeout(Some(Duration::from_secs(timeout))).expect("set write timeout failed!");
    socket.set_read_timeout(Some(Duration::from_secs(timeout))).expect("set read timeout failed!");

    let mut buf = vec![0; size];

    let bytes = socket.write(&buf).expect("Send failed!");
    println!("{} bytes sent to {} through single package, remain {} bytes", bytes, addr, size - bytes);

    let t = std::time::SystemTime::now();

    let bytes = socket.read(&mut buf).expect("Receive failed!");
    println!("{} bytes received from {} through single package, remain {} bytes", bytes, addr, size - bytes);

    let dur = t.elapsed().expect("Can't elapsed!");
    println!("ping: {}ms", dur.as_millis());
}