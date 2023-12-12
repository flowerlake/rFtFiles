use std::error::Error;
use std::fs::File;
use std::{io, thread};
use std::io::{Read, Write};
use std::net::{Ipv6Addr, SocketAddrV6, TcpListener};
use std::thread::Thread;

fn get_file_content(filepath: &str) -> Result<Vec<u8>, Box<dyn Error>> {
    let mut file = File::open(filepath)?;

    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)?;

    Ok(buffer)
}

fn handle_client(mut stream: std::net::TcpStream) {
    // 处理连接的请求
    let mut buffer = [0; 128];
    let bytes_read = stream.read(&mut buffer).expect("Failed to read from client");
    let message = String::from_utf8_lossy(&buffer[..bytes_read]);
    println!("Received message from client: {}", message);

    // 回应客户端
    let response = "Hello, client!";
    stream.write_all(response.as_bytes()).expect("Failed to write to client");
}


fn main() -> io::Result<()> {

    let listen_addr = SocketAddrV6::new(Ipv6Addr::UNSPECIFIED, 48520, 0, 0);
    let listen = TcpListener::bind(&listen_addr)?;
    for stream in listen.incoming() {
        match stream {
            Ok(stream) => {
                // thread::spawn();
                handle_client(stream);
            }
            Err(e) => {
                eprintln!("Failed to accept connection {}", e);
            }
        }
    }
    Ok(())
}
