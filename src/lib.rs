use std::net::*;
use std::str::FromStr;
use trust_dns::client::{Client, SyncClient};
use trust_dns::udp::UdpClientConnection;
use trust_dns::op::DnsResponse;
use trust_dns::rr::{DNSClass, Name, RData, Record, RecordType};
use error_chain::{error_chain, ChainedError, bail};

error_chain! {
    errors {
        NoResponse {
            description("didn't receive response.")
        }
    }
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
    pub fn query_ipv4(&self) -> Result<Ipv4Addr> {
        match self {
            Provider::OpenDNS => self.query_ipv4_opendns(),
        }
    }

    pub fn query_ipv6(&self) -> Result<Ipv6Addr> {
        match self {
            Provider::OpenDNS => self.query_ipv6_opendns(),
        }
    }

    fn query_ipv4_opendns(&self) -> Result<Ipv4Addr> {
        let address = "208.67.222.222:53".parse().unwrap();
        let conn = UdpClientConnection::new(address).unwrap();
        let client = SyncClient::new(conn);

        let name = Name::from_str("myip.opendns.com.").unwrap();

        let response: DnsResponse = client.query(&name, DNSClass::IN, RecordType::A).unwrap();

        let answers: &[Record] = response.answers();

        if let &RData::A(ref ip) = answers[0].rdata() {
            Ok(*ip)
        } else {
            bail!(ErrorKind::NoResponse)
        }
    }

    fn query_ipv6_opendns(&self) -> Result<Ipv6Addr> {
        let address = "[2620:119:35::35]:53".parse().chain_err(|| "")?;
        let conn = UdpClientConnection::new(address).unwrap();
        let client = SyncClient::new(conn);

        let name = Name::from_str("myip.opendns.com.").unwrap();

        let response: DnsResponse = client.query(&name, DNSClass::IN, RecordType::AAAA).unwrap();

        let answers: &[Record] = response.answers();

        if let &RData::AAAA(ref ip) = answers[0].rdata() {
            Ok(*ip)
        } else {
            bail!(ErrorKind::NoResponse)
        }
    }
}
