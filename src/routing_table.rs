use std::{collections::HashMap, net::IpAddr, process::{Command, ExitStatus}, string::FromUtf8Error};

use crate::RouteEntry;

const NETSTAT_PATH: &str = "/usr/sbin/netstat";

pub struct RoutingTable {
    routes: Vec<RouteEntry>,
    /// Map of interfaces to their default routers
    if_router: HashMap<String, Vec<IpAddr>>,
}

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("failed to execute {NETSTAT_PATH}: {0}")]
    NetstatExec(std::io::Error),
    #[error("failed to get routing table: {0}")]
    NetstatFail(ExitStatus),
    #[error("netstat output not non-UTF-8")]
    NetstatUtf8(FromUtf8Error),
    #[error("no headers follow {0:?} section marker")]
    NetstatParseNoHeaders(String),
    #[error("parsing route entry: {0}")]
    RouteEntryParse(#[from] crate::route_entry::Error),
    #[error("route entry found before protocol (Internet/Internet6) found.")]
    EntryBeforeProto,
}

impl RoutingTable {
    pub fn find_route_entry(&self, addr: IpAddr) -> Option<&RouteEntry> {
        // TODO: implement a proper lookup table and/or short-circuit on an
        // exact match
        todo!()
    }
}

pub async fn execute_netstat() -> Result<String, Error> {
    let output = Command::new(NETSTAT_PATH)
        .arg("-rn")
        .stdin(std::process::Stdio::null())
        .output()
        .map_err(Error::NetstatExec)?;
    if !output.status.success() {
        return Err(Error::NetstatFail(output.status));
    }
    String::from_utf8(output.stdout).map_err(Error::NetstatUtf8)
}