use std::net::*;
use trust_dns_resolver::Resolver;
use trust_dns_resolver::config::*;
use std::str::FromStr;
use failure::*;

#[derive(Fail, Debug)]
enum Errors {
    #[fail(display = "No response.")]
    NoResponse,
}

pub enum Provider {
    Google,
}

impl Default for Provider {
    fn default() -> Provider {
        Provider::Google
    }
}

impl Provider {
    pub fn query_ip(&self) -> Result<IpAddr, Error> {
        match self {
            Provider::Google => self.query_ip_google(),
        }
    }

    fn query_ip_google(&self) -> Result<IpAddr, Error> {
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
