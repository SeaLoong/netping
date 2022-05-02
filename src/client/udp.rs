use std::net::{SocketAddr, UdpSocket};
use std::time::Duration;

pub fn ping(addr: SocketAddr, size: usize, timeout: u64) {
    let mut port = 20000;
    let socket = loop {
        if let Ok(socket) = UdpSocket::bind(SocketAddr::from(([127, 0, 0, 1], port))) {
            break socket;
        }
        port += 1;
    };

    socket
        .set_write_timeout(Some(Duration::from_secs(timeout)))
        .expect("set write timeout failed!");
    socket
        .set_read_timeout(Some(Duration::from_secs(timeout)))
        .expect("set read timeout failed!");

    let mut buf = vec![0; size];

    let bytes = socket.send_to(&buf, addr).expect("Send failed!");
    println!("{} bytes sent to {}", bytes, addr);

    let t = std::time::SystemTime::now();
    loop {
        let (bytes, src_addr) = socket.recv_from(&mut buf).expect("Receive failed!");
        if src_addr == addr {
            println!("{} bytes received from {}", bytes, src_addr);

            let dur = t.elapsed().expect("Can't elapsed!");
            println!("ping: {}ms", dur.as_millis());
            break;
        }
    }
}
