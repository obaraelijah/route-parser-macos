use std::{collections::HashSet, fmt::Error, time::Duration};

use crate::{Destination, Protocol, RoutingFlag};

pub struct RouteEntry {
    pub proto: Protocol,
    pub dest: Destination,
    pub gateaway: Destination,
    pub flags: HashSet<RoutingFlag>,
    pub net_if: String,
    pub expires: Option<Duration>,
}

impl RouteEntry {
    pub(crate) fn parse(proto: Protocol, line: &str) -> Result<Self, Error> {
        todo!()
    }
}
