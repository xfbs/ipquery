use ipquery::*;

fn main() {
    let provider = Provider::default();
    match provider.query_ipv4() {
        Ok(ip) => println!("IPv4 = {}", ip),
        Err(e) => println!("IPv4 lookup error: {}", e),
    }

    match provider.query_ipv6() {
        Ok(ip) => println!("IPv6 = {}", ip),
        Err(e) => println!("IPv6 lookup error: {}", e),
    }

    let provider = DnsIpProvider::default();
    match provider.query_ip() {
        Ok(ip) => println!("DNS IP = {}", ip),
        Err(e) => println!("DNS IP error: {}", e)
    }
}
