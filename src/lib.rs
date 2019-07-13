use std::net::{Ipv4Addr, Ipv6Addr, IpAddr};
use failure::Error;

pub mod opendns;
pub mod google;

pub trait GetIpv4 {
    fn query_ipv4(&self) -> Result<Ipv4Addr, Error>;
}

pub trait GetIpv6 {
    fn query_ipv6(&self) -> Result<Ipv6Addr, Error>;
}

pub trait GetDnsIp {
    fn query_dns_ip(&self) -> Result<IpAddr, Error>;
}
