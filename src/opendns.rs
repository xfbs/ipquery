use std::net::*;
use std::str::FromStr;
use trust_dns::client::{Client, SyncClient};
use trust_dns::op::DnsResponse;
use trust_dns::rr::{DNSClass, Name, RData, Record, RecordType};
use trust_dns::udp::UdpClientConnection;
use failure::{Fail, Error, bail};
use crate::{GetIpv4, GetIpv6};

#[derive(Fail, Debug)]
enum Errors {
    #[fail(display = "No response.")]
    NoResponse,
}

pub enum OpenDNS {
    Resolver1,
    Resolver2,
    Resolver3,
    Resolver4,
}

impl OpenDNS {
    pub fn new(resolver: usize) -> Option<OpenDNS> {
        use OpenDNS::*;
        match resolver {
            1 => Some(Resolver1),
            2 => Some(Resolver2),
            3 => Some(Resolver3),
            4 => Some(Resolver4),
            _ => None,
        }
    }

    pub fn ipv4_addr(&self) -> Ipv4Addr {
        use OpenDNS::*;
        match self {
            Resolver1 => Ipv4Addr::new(208, 67, 222, 222),
            Resolver2 => Ipv4Addr::new(208, 67, 220, 220),
            Resolver3 => Ipv4Addr::new(208, 67, 222, 220),
            Resolver4 => Ipv4Addr::new(208, 67, 220, 222),
        }
    }

    pub fn ipv6_addr(&self) -> Ipv6Addr {
        // FIXME: resolver3 and 4 don't have IPv6.
        use OpenDNS::*;
        match self {
            Resolver1 => Ipv6Addr::new(0x2620, 0x0119, 0x0035, 0, 0, 0, 0, 0x0035),
            Resolver2 => Ipv6Addr::new(0x2620, 0x0119, 0x0035, 0, 0, 0, 0, 0x0053),
            Resolver3 => Ipv6Addr::new(0x2620, 0x0119, 0x0035, 0, 0, 0, 0, 0x0035),
            Resolver4 => Ipv6Addr::new(0x2620, 0x0119, 0x0035, 0, 0, 0, 0, 0x0035),
        }
    }
}

impl GetIpv4 for OpenDNS {
    fn query_ipv4(&self) -> Result<Ipv4Addr, Error> {
        let address = SocketAddr::new(IpAddr::V4(self.ipv4_addr()), 53);
        let conn = UdpClientConnection::new(address)?;
        let client = SyncClient::new(conn);
        let name = Name::from_str("myip.opendns.com.")?;
        let response: DnsResponse = client.query(&name, DNSClass::IN, RecordType::A)?;
        let answers: &[Record] = response.answers();

        if let &RData::A(ref ip) = answers[0].rdata() {
            Ok(*ip)
        } else {
            bail!(Errors::NoResponse)
        }
    }
}

impl GetIpv6 for OpenDNS {
    fn query_ipv6(&self) -> Result<Ipv6Addr, Error> {
        let address = SocketAddr::new(IpAddr::V6(self.ipv6_addr()), 53);
        let conn = UdpClientConnection::new(address)?;
        let client = SyncClient::new(conn);
        let name = Name::from_str("myip.opendns.com.")?;
        let response: DnsResponse = client.query(&name, DNSClass::IN, RecordType::AAAA)?;
        let answers: &[Record] = response.answers();

        if let &RData::AAAA(ref ip) = answers[0].rdata() {
            Ok(*ip)
        } else {
            bail!(Errors::NoResponse)
        }
    }
}
