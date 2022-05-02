pub mod client {
    pub mod tcp;
    pub mod udp;
}

pub mod server {
    pub mod tcp;
    pub mod udp;
}

pub mod dns_resolver;

use clap::Parser;
use std::net::SocketAddr;
use std::str::FromStr;

/// Simple network test program. TCP/UDP ping, TCP/UDP server.
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Host and Port
    #[clap(name = "host:port")]
    address: String,

    /// Using UDP socket. Default using TCP socket.
    #[clap(short, long)]
    udp: bool,

    /// Run as server. Default run as client.
    #[clap(short, long)]
    server: bool,

    /// Package size (bytes).
    #[clap(short = 'S', long, default_value_t = 1024)]
    size: usize,

    /// Timeout (second) for client waiting.
    #[clap(short = 'T', long, default_value_t = 10)]
    timeout: u64,
}

fn main() {
    let args = Args::parse();

    let address: &str = &args.address;
    let mut split = address.split(':');
    let host = split.next().expect("Invalid address!");
    let port = split.next().expect("Invalid address!");
    let addr = SocketAddr::new(
        dns_resolver::lookup(host),
        u16::from_str(port).expect("Invalid port!"),
    );

    if args.server {
        if args.udp {
            server::udp::start(addr, args.size, args.timeout);
        } else {
            server::tcp::start(addr, args.size, args.timeout);
        }
    } else if args.udp {
        client::udp::ping(addr, args.size, args.timeout);
    } else {
        client::tcp::ping(addr, args.size, args.timeout);
    }
}
