use std::net::{SocketAddr, UdpSocket};
use std::str::FromStr;
use std::time::Duration;


pub fn start(addr: &str, size: usize, timeout: u64) {
    let addr = SocketAddr::from_str(addr).expect("Invalid address!");
    let socket = UdpSocket::bind(addr).expect("Bind address failed!");
    println!("UDP Server is listening on {}", addr);
    let mut buf = vec![0; size];
    loop {
        let (bytes, src_addr) = socket.recv_from(&mut buf).expect("Receive failed!");
        println!("{} bytes received from {}", bytes, src_addr);
        socket.set_write_timeout(Some(Duration::from_secs(timeout))).expect("set write timeout failed!");
        let rem = socket.send_to(&buf, src_addr).expect("Send failed!");
        println!("{} bytes sent to {}", rem, src_addr);
    }
}