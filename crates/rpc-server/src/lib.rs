use anyhow::Result;
use hasher::tx_hash;
use jsonrpsee::{
    RpcModule,
    server::{ServerBuilder, ServerHandle},
    types::{ErrorCode, ErrorObjectOwned},
};
use std::net::SocketAddr;
use types::transaction::Transaction;
pub struct RpcImpl {}

pub async fn run_http(addr: SocketAddr) -> Result<ServerHandle> {
    let server = ServerBuilder::default().build(addr).await?;

    let mut module = RpcModule::new(());
    module.register_method("ping", |_, _, _| "pong")?;
    module.register_method(
        "submit_transaction",
        |params, _, _| -> Result<String, ErrorObjectOwned> {
            let tx: Transaction = params.one().map_err(|e| {
                ErrorObjectOwned::owned(ErrorCode::InvalidParams.code(), e.to_string(), None::<()>)
            })?;
            let hash = format!("0x{}", tx_hash(&tx));
            Ok(hash)
        },
    )?;

    let handle = server.start(module);
    Ok(handle)
}
