use anyhow::Result;
use route_parser_macos::{RouteEntry, RoutingTable};

#[tokio::main]
async fn main() -> Result<()> {
    let rt = RoutingTable::load_from_netstat().await?;
    let addr = "1.1.1.1".parse()?;

    if let Some(RouteEntry {
        net_if, gateway, ..
    }) = rt.find_route_entry(addr)
    {
        println!("{addr:?} => {gateway} via {net_if}");
    } else {
        println!("No route to {addr:?}");
    }

    Ok(())
}
