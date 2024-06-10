mod route_entry;
mod routing_flag;

use std::fmt::Write;

// Exports
pub use route_entry::RouteEntry;
pub use routing_flag::RoutingFlag;

use cidr::AnyIpCidr; //  either an IPv4 or an IPv6 network or "any".
use mac_address::MacAddress;

/// A generic network entity
#[derive(Debug, Clone)]
pub enum Entity {
    Default,
    Cidr(AnyIpCidr),
    Link(String),
    Mac(MacAddress),
}

impl std::fmt::Display for Entity {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Entity::Default => f.write_str("default"),
            Entity::Cidr(cidr) => write!(f, "{cidr}"),
            Entity::Link(link) => f.write_str(link),
            Entity::Mac(mac) => {
                for (i, byte) in mac.bytes().iter().enumerate() {
                    if i > 0 {
                        f.write_char(':')?;
                    }
                    write!(f, "{byte:02x}")?;
                }
                Ok(())
            }
        }
    }
}


pub struct Destination {
    pub entity: Entity,
}

impl std::fmt::Display for Destination {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let Destination { entity } = self;
        write!(f, "{entity}")?;
        Ok(())
    }
}

pub enum Protocol {
    V4,
    V6,
}
