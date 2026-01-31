use anyhow::{Ok, Result};
use jsonrpsee::{
    RpcModule,
    server::{ServerBuilder, ServerHandle},
};
use std::net::SocketAddr;
pub struct RpcImpl {}

pub async fn run_http(addr: SocketAddr) -> Result<ServerHandle> {
    let server = ServerBuilder::default().build(addr).await?;

    let mut module = RpcModule::new(());
    module.register_method("ping", |_, _, _| "pong")?;

    let handle = server.start(module);
    Ok(handle)
}
