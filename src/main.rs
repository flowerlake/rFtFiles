use std::net::{Ipv6Addr, SocketAddrV6, TcpListener};
use std::{io, thread};

use libr_fet::*;

fn main() -> io::Result<()> {
    let listen_port = 48520;
    let ipv6_addr = SocketAddrV6::new(Ipv6Addr::UNSPECIFIED, listen_port, 0, 0);
    let listener = TcpListener::bind(&ipv6_addr)?;
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                thread::spawn(|| util::network_tool::handle_client(stream));
            }
            Err(e) => {
                eprintln!("Failed to accept connection {}", e);
            }
        }
    }
    Ok(())
}
