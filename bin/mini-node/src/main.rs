use anyhow::Result;
use std::net::SocketAddr;

#[tokio::main]
async fn main() -> Result<()> {
    let addr: SocketAddr = "127.0.0.1:8080".parse().unwrap();
    let handle = rpc_server::run_http(addr).await?;
    println!("JSON-RPC server listening on http://{addr:?}");

    //keep process alive until Ctrl+c
    tokio::signal::ctrl_c().await?;
    println!("Shutting down...");

    handle.stop()?;
    handle.stopped().await;
    Ok(())
}
