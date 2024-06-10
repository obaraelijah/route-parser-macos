use std::{collections::HashSet, time::Duration};

use crate::{Destination, Protocol, RoutingFlag};

/// A single route obtained from the `netstat -rn` output
#[derive(Debug, Clone)]
pub struct RouteEntry {
    /// Protocol
    pub proto: Protocol,

    /// Destination.  E.g., a host or CIDR
    pub dest: Destination,

    /// Gateway (i.e., how to reach the destination)
    pub gateway: Destination,

    /// Routing flags
    pub flags: HashSet<RoutingFlag>,

    /// Network interface that holds this route
    pub net_if: String,

    /// RouteEntry expiration.  This is primarily seen for ARP-derived entries
    pub expires: Option<Duration>,
}

impl std::fmt::Display for RouteEntry {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        #[allow(unused_variables)]
        let RouteEntry {
            proto,
            dest,
            gateway,
            flags,
            net_if,
            expires,
        } = self;
        write!(f, "{proto:?}({dest} -> {gateway} if={net_if}")
    }
}

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("parsing destination CIDR {value:?}: {err}")]
    ParseDestination {
        value: String,
        err: cidr::errors::NetworkParseError,
    },

    #[error("parsing MAC addr {dest:?}: {err}")]
    ParseMacAddr {
        dest: String,
        err: mac_address::MacParseError,
    },

    #[error("unparseable byte in IPv4 address {addr:?}: {err}")]
    ParseIPv4AddrBadInt {
        addr: String,
        err: std::num::ParseIntError,
    },

    #[error("invalid number of IPv4 address components ({n_comps}) in {addr:?}")]
    ParseIPv4AddrNComps { n_comps: usize, addr: String },

    #[error("invalid expiration {expiration:?}: {err}")]
    ParseExpiration {
        expiration: String,
        err: std::num::ParseIntError,
    },

    #[error("missing destination")]
    MissingDestination,

    #[error("missing gateway")]
    MissingGateway,

    #[error("missing network interface")]
    MissingInterface,
}

impl RouteEntry {
    pub(crate) fn parse(proto: Protocol, line: &str) -> Result<Self, Error> {
        let flags = HashSet::new();
        let dest = None;
        let gateway = None;
        let net_if: Option<String> = None;
        let expires = None;

        let route = RouteEntry {
            proto,
            dest: dest.ok_or(Error::MissingDestination)?,
            gateway: gateway.ok_or(Error::MissingGateway)?,
            flags,
            net_if: net_if.ok_or(Error::MissingInterface)?,
            expires,
       };
       Ok(route)
    }
}

fn parse_flags(flag_s: &str) -> HashSet<RoutingFlag> {
    flag_s.chars().map(RoutingFlag::from).collect()
}


