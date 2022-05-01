
pub mod client {
    pub mod tcp;
    pub mod udp;
}

pub mod server {
    pub mod tcp;
    pub mod udp;
}

use clap::Parser;

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
    if args.server {
        if args.udp {
            server::udp::start(&args.address, args.size, args.timeout);
        } else {
            server::tcp::start(&args.address, args.size, args.timeout);
        }
    } else {
        if args.udp {
            client::udp::ping(&args.address, args.size, args.timeout);
        } else {
            client::tcp::ping(&args.address, args.size, args.timeout);
        }
    }
}
