use failure::{Error, Fail, bail};
use crate::*;
use ::pnet::datalink::interfaces;

#[derive(Debug, Fail)]
pub enum Errors {
    #[fail(display = "No interface or no public IP.")]
    NoInterface,
}

pub enum Pnet {
    Default,
    Interface(String),
}

impl GetIpv4 for Pnet {
    fn query_ipv4(&self) -> Result<Ipv4Addr, Error> {
        for interface in interfaces() {
            for ip in interface.ips {
                match ip.ip() {
                    IpAddr::V4(ip) if ip.is_global() => return Ok(ip),
                    _ => {}
                }
            }
        }

        bail!(Errors::NoInterface);
    }
}

impl GetIpv6 for Pnet {
    fn query_ipv6(&self) -> Result<Ipv6Addr, Error> {
        for interface in interfaces() {
            for ip in interface.ips {
                match ip.ip() {
                    IpAddr::V6(ip) if ip.is_global() => return Ok(ip),
                    _ => {}
                }
            }
        }

        bail!(Errors::NoInterface);
    }
}
