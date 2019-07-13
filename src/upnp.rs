use std::net::Ipv4Addr;
use igd::search_gateway;
use failure::Error;
use crate::GetIpv4;

#[derive(Debug, Clone, Copy)]
pub enum Upnp {
    Default
}

impl GetIpv4 for Upnp {
    fn query_ipv4(&self) -> Result<Ipv4Addr, Error> {
        let gateway = search_gateway(Default::default())?;
        let ip = gateway.get_external_ip()?;
        Ok(ip)
    }
}
