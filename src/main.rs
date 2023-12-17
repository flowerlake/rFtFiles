use std::error::Error;
use std::fs::File;
use std::io::Read;
use std::net::{Ipv6Addr, SocketAddrV6, TcpListener};
use std::{io, thread};

use ipv6_tool;

use libr_fet::*;

fn get_file_content(filepath: &str) -> Result<Vec<u8>, Box<dyn Error>> {
    let mut file = File::open(filepath)?;

    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)?;

    Ok(buffer)
}

fn main() -> io::Result<()> {
    let ipv6_addr = SocketAddrV6::new(Ipv6Addr::UNSPECIFIED, 48520, 0, 0);
    let listener = TcpListener::bind(&ipv6_addr)?;
    let test_tool_func = ipv6_tool::add_one(5);
    println!("{}", test_tool_func);

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
