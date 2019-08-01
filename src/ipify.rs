use url::Url;
use failure::{Error, Fail, bail};
use crate::*;
use reqwest::Client;

#[derive(Debug, Fail)]
pub enum Errors {
    #[fail(display = "No response received.")]
    NoResponse,
    #[fail(display = "Can't parse response '{}'.", _0)]
    AddrParseError(String),
}

pub enum Ipify {
    Default,
}

impl GetIpv4 for Ipify {
    fn query_ipv4(&self) -> Result<Ipv4Addr, Error> {
        // create url with ipv4 address.
        let url = Url::parse("http://api.ipify.org/")?;

        // perform a get request.
        let client = Client::builder()
            .build()?;
        let mut resp = client.get(url).send()?;
        let body = resp.text()?;

        // parse and return ipv4.
        let address = body.parse::<Ipv4Addr>()?;
        Ok(address)
    }
}

impl GetIpv6 for Ipify {
    fn query_ipv6(&self) -> Result<Ipv6Addr, Error> {
        // create url with ipv4 address.
        let url = Url::parse("http://api6.ipify.org/")?;

        // perform a get request.
        let client = Client::builder()
            .build()?;
        let mut resp = client.get(url).send()?;
        let body = resp.text()?;

        // parse and return ipv4.
        if let Ok(address) = body.parse::<Ipv6Addr>() {
            Ok(address)
        } else {
            bail!(Errors::AddrParseError(body));
        }
    }
}
