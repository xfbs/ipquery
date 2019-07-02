use std::net::*;
use std::str::FromStr;
use trust_dns::client::{Client, SyncClient};
use trust_dns::op::DnsResponse;
use trust_dns::rr::{DNSClass, Name, RData, Record, RecordType};
use trust_dns::udp::UdpClientConnection;
use failure::*;

mod dns_ip;
pub use dns_ip::{Provider as DnsIpProvider};

#[derive(Fail, Debug)]
enum Errors {
    #[fail(display = "No response.")]
    NoResponse,
}

pub enum Provider {
    OpenDNS,
}

impl Default for Provider {
    fn default() -> Provider {
        Provider::OpenDNS
    }
}

impl Provider {
    pub fn query_ipv4(&self) -> Result<Ipv4Addr, Error> {
        match self {
            Provider::OpenDNS => self.query_ipv4_opendns(),
        }
    }

    pub fn query_ipv6(&self) -> Result<Ipv6Addr, Error> {
        match self {
            Provider::OpenDNS => self.query_ipv6_opendns(),
        }
    }

    fn query_ipv4_opendns(&self) -> Result<Ipv4Addr, Error> {
        let address = "208.67.222.222:53".parse()?;
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

    fn query_ipv6_opendns(&self) -> Result<Ipv6Addr, Error> {
        let address = "[2620:119:35::35]:53".parse()?;
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
