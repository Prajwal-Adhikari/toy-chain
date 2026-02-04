use anyhow::Result;
use clap::Parser;
use std::net::SocketAddr;

#[derive(Debug, Parser)]
struct Cli {
    #[arg(long, default_value = "127.0.0.1:8080")]
    pub rpc_addr: SocketAddr,
}
#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    let handle = rpc_server::run_http(cli.rpc_addr).await?;
    println!("JSON-RPC server listening on http://{}", cli.rpc_addr);

    //keep process alive until Ctrl+c
    tokio::signal::ctrl_c().await?;
    println!("Shutting down...");

    handle.stop()?;
    handle.stopped().await;
    Ok(())
}
