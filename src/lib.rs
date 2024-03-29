#![feature(ip)]

use std::net::{Ipv4Addr, Ipv6Addr, IpAddr};
use failure::Error;

pub mod opendns;
pub mod google;
pub mod upnp;
pub mod codeux;
pub mod ipify;
pub mod pnet;

pub use opendns::OpenDNS;
pub use google::Google;
pub use upnp::Upnp;
pub use codeux::Codeux;
pub use ipify::Ipify;
pub use crate::pnet::Pnet;

pub trait GetIpv4 {
    fn query_ipv4(&self) -> Result<Ipv4Addr, Error>;
}

pub trait GetIpv6 {
    fn query_ipv6(&self) -> Result<Ipv6Addr, Error>;
}

pub trait GetDnsIp {
    fn query_dns_ip(&self) -> Result<IpAddr, Error>;
}

pub fn get_ipv4_query(name: &str) -> Option<Box<dyn GetIpv4>> {
    match name {
        "opendns1" => Some(Box::new(OpenDNS::Resolver1)),
        "opendns2" => Some(Box::new(OpenDNS::Resolver2)),
        "opendns3" => Some(Box::new(OpenDNS::Resolver3)),
        "opendns4" => Some(Box::new(OpenDNS::Resolver4)),
        "codeux" => Some(Box::new(Codeux::Default)),
        "upnp" => Some(Box::new(Upnp::Default)),
        "ipify" => Some(Box::new(Ipify::Default)),
        "pnet" => Some(Box::new(Pnet::Default)),
        _ => None,
    }
}

pub fn get_ipv6_query(name: &str) -> Option<Box<dyn GetIpv6>> {
    match name {
        "ipify" => Some(Box::new(Ipify::Default)),
        "pnet" => Some(Box::new(Pnet::Default)),
        _ => None,
    }
}
