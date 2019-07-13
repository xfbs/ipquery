use ipquery::*;
use clap::{App, Arg};

fn main() {
    let matches = App::new("xar")
        .version(env!("CARGO_PKG_VERSION"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .about(env!("CARGO_PKG_DESCRIPTION"))
        .arg(
            Arg::with_name("v")
                .short("v")
                .multiple(true)
                .help("Sets the level of verbosity"),
        )
        .arg(
            Arg::with_name("ipv4")
                .short("4")
                .long("ipv4")
                .takes_value(true)
                .default_value("opendns")
                .help("Lookup the IPv4 address using the given method."),
        )
        .arg(
            Arg::with_name("ipv6")
                .short("6")
                .long("ipv6")
                .takes_value(true)
                .default_value("opendns")
                .help("Lookup the IPv6 address using the given method."),
        )
        .arg(
            Arg::with_name("dns")
                .short("d")
                .long("dns")
                .takes_value(true)
                .default_value("opendns")
                .help("Lookup the DNS resolver IP address using the given method."),
        )
        .get_matches();

    if matches.occurrences_of("ipv4") > 0 {
        let provider = opendns::OpenDNS::Resolver1;
        match provider.query_ipv4() {
            Ok(ip) => println!("IPv4 = {}", ip),
            Err(e) => println!("IPv4 lookup error: {}", e),
        }
    }

    if matches.occurrences_of("ipv6") > 0 {
        let provider = opendns::OpenDNS::Resolver1;
        match provider.query_ipv6() {
            Ok(ip) => println!("IPv6 = {}", ip),
            Err(e) => println!("IPv6 lookup error: {}", e),
        }
    }

    if matches.occurrences_of("dns") > 0 {
        let provider = DnsIpProvider::default();
        match provider.query_ip() {
            Ok(ip) => println!("DNS IP = {}", ip),
            Err(e) => println!("DNS IP error: {}", e)
        }
    }
}
