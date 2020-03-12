use std::net::IpAddr;
use std::str::FromStr;

use crate::base::{Error, Result};

pub struct IPAddress(IpAddr);

pub enum IPAddressKind {
    IPV4,
    IPV6,
}

impl IPAddress {
    pub fn kind(&self) -> IPAddressKind {
        match self {
            IPAddress(IpAddr::V4(_)) => IPAddressKind::IPV4,
            IPAddress(IpAddr::V6(_)) => IPAddressKind::IPV6,
        }
    }
}

impl ToString for IPAddress {
    fn to_string(&self) -> String {
        let IPAddress(addr) = self;
        addr.to_string()
    }
}

impl FromStr for IPAddress {
    type Err = Error;

    fn from_str(s: &str) -> Result<IPAddress> {
        s.parse().map(IPAddress).map_err(Into::into)
    }
}
