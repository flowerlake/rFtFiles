use std::net::{IpAddr, ToSocketAddrs};

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

pub fn domain2ipv6(domain: &str) -> Result<String, String> {
    // 尝试将域名解析为 Socket 地址
    let addrs = domain.to_socket_addrs();

    match addrs {
        Ok(socket_addrs) => {
            // 遍历解析到的地址，找到 IPv6 地址
            for addr in socket_addrs {
                if let IpAddr::V6(ipv6_addr) = addr.ip() {
                    // 返回 IPv6 地址的字符串表示
                    return Ok(ipv6_addr.to_string());
                }
            }
            // 如果没有找到 IPv6 地址，返回错误信息
            Err("No IPv6 address found for the domain".to_string())
        }
        Err(e) => {
            // 处理解析错误，返回错误信息
            Err(format!("Failed to resolve domain: {}", e))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    #[test]
    fn test2() {
        match domain2ipv6("jellfy.flowerlake23.top:39696") {
            Ok(ipv6) => println!("IPv6 address: {}", ipv6),
            Err(err) => eprintln!("Error: {}", err),
        }
    }
}
