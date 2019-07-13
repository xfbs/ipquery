use std::net::*;
use trust_dns_resolver::Resolver;
use trust_dns_resolver::config::*;
use std::str::FromStr;
use failure::*;
use crate::*;

#[derive(Fail, Debug)]
enum Errors {
    #[fail(display = "No response.")]
    NoResponse,
}

pub enum Google {
    Default,
}

impl Google {
    pub fn new() -> Google {
        Google::Default
    }
}

impl GetDnsIp for Google {
    fn query_dns_ip(&self) -> Result<IpAddr, Error> {
        let resolver = Resolver::new(ResolverConfig::default(), ResolverOpts::default()).unwrap();
        // let mut resolver = Resolver::from_system_conf().unwrap();
        let response = resolver.txt_lookup("o-o.myaddr.google.com.")?;

        for data in response.iter() {
            for addr in data.iter() {
                let addr = String::from_utf8_lossy(&addr);
                let addr = IpAddr::from_str(&addr)?;
                return Ok(addr);
            }
        }

        bail!(Errors::NoResponse)
    }
}
