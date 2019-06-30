use std::net::*;
use trust_dns_resolver::Resolver;
use trust_dns_resolver::config::*;

pub enum Provider {
    OpenDNS,
}

impl Default for Provider {
    fn default() -> Provider {
        Provider::OpenDNS
    }
}

impl Provider {
    pub fn query_ipv4(&self) -> Option<Ipv4Addr> {
        match self {
            Provider::OpenDNS => self.query_ipv4_opendns(),
        }
    }

    pub fn query_ipv6() -> Option<Ipv6Addr> {
        None
    }

    fn query_ipv4_opendns(&self) -> Option<Ipv4Addr> {
        None
    }
}
