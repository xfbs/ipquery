use trust_dns_resolver::Resolver;
use url::Url;
use failure::{Error, Fail};
use crate::*;
use reqwest::Client;

#[derive(Debug, Fail)]
pub enum Errors {
    #[fail(display = "No response received.")]
    NoResponse
}

pub enum Codeux {
    Default,
}

impl GetIpv4 for Codeux {
    fn query_ipv4(&self) -> Result<Ipv4Addr, Error> {
        // lookup ipv4 of domain.
        let resolver = Resolver::default()?;
        let response = resolver.ipv4_lookup("myip.codeux.com.")?;
        let address = response.iter().nth(0).ok_or(Errors::NoResponse)?;

        // create url with ipv4 address.
        let mut url = Url::parse("https://myip.codeux.com/")?;
        url.set_ip_host(IpAddr::V4(*address)).unwrap();

        // perform a get request.
        let client = Client::new();
        let mut resp = client.get(url).send()?;
        let body = resp.text()?;

        // parse and return ipv4.
        let address = body.parse::<Ipv4Addr>()?;
        Ok(address)
    }
}
