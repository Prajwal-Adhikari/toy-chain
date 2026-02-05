use anyhow::{Context, Result};
use clap::Parser;
use config::config::AppConfig;
use config_rs::{Config as Cfg, Environment, File};
use std::net::SocketAddr;
#[derive(Debug, Parser)]
struct Cli {
    //path to config file (TOML)
    #[arg(long, default_value = "config.toml")]
    pub config: String,

    #[arg(long, default_value = "127.0.0.1:8080")]
    pub rpc_addr: SocketAddr,
}

fn load_app_config(cli: &Cli) -> Result<AppConfig> {
    //1. Build layered config: file then env overrides

    let built = Cfg::builder()
        .add_source(File::with_name(&cli.config).required(true))
        .add_source(Environment::with_prefix("APP").separator("__"))
        .build()
        .context("building configuration sources")?;

    //2. Deserialize into typed struct
    let mut cfg: AppConfig = built
        .try_deserialize()
        .context("desearializing config into AppConfig")?;

    //3. Apply CLI overrides (CLI wins)
    cfg.rpc.addr = cli.rpc_addr;

    //4. validate invariants
    cfg.validate().context("validating config")?;

    Ok(cfg)
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();
    let my_config = load_app_config(&cli);
    let handle = rpc_server::run_http(cli.rpc_addr).await?;
    println!("JSON-RPC server listening on http://{}", cli.rpc_addr);

    //keep process alive until Ctrl+c
    tokio::signal::ctrl_c().await?;
    println!("Shutting down...");

    handle.stop()?;
    handle.stopped().await;
    Ok(())
}
