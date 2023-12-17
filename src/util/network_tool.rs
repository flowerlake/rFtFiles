use std::io::{Read, Write};

pub fn handle_client(mut stream: std::net::TcpStream) {
    // 处理连接的请求
    let mut buffer = [0; 128];
    let bytes_read = stream
        .read(&mut buffer)
        .expect("Failed to read from client");

    // 如果有读取到数据，进行处理
    if bytes_read > 0 {
        // 打印接收到的字节
        println!("Received bytes from client: {:?}", &buffer[..bytes_read]);

        // 这里可以对接收到的数据进行进一步处理
        // 例如，将字节转换为字符串
        let received_str = String::from_utf8_lossy(&buffer[..bytes_read]);
        println!("Received message from client: {}", received_str);

        // 回应客户端
        let response = "Hello, client!";
        stream
            .write_all(response.as_bytes())
            .expect("Failed to write to client");
    }
}
