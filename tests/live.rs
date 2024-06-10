#![allow(clippy::missing_panics_doc, clippy::missing_errors_doc)]

use anyhow::Result;
use route_parser_macos::{execute_netstat, RoutingTable};

#[tokio::test]
pub async fn main() -> Result<()> {
    let raw = execute_netstat().await?;
    eprintln!("Raw Table:");
    eprintln!("----------");
    eprint!("{raw}");
    eprintln!("----------");
    let table = RoutingTable::load_from_netstat().await?;
    eprintln!("{table:?}");

    Ok(())
}