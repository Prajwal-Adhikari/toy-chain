use anyhow::Result;
use hasher::tx_hash;
use jsonrpsee::{
    RpcModule,
    server::{ServerBuilder, ServerHandle},
    types::{ErrorCode, ErrorObjectOwned},
};
use std::{net::SocketAddr, sync::Arc};
use tokio::sync::Mutex;
use txpool::{InsertionOutcome, RejectReason, TxPool};
use types::transaction::Transaction;
type SharedTxPool = Arc<Mutex<TxPool>>;
pub struct RpcImpl {}

pub async fn run_http(addr: SocketAddr) -> Result<ServerHandle> {
    let server = ServerBuilder::default().build(addr).await?;

    let pool: SharedTxPool = Arc::new(Mutex::new(TxPool::new()));

    let mut module = RpcModule::new(pool.clone());
    module.register_method("ping", |_, _, _| "pong")?;
    module.register_async_method("submit_transaction", |params, pool, _| async move {
        let tx: Transaction = params.one().map_err(|e| {
            ErrorObjectOwned::owned(ErrorCode::InvalidParams.code(), e.to_string(), None::<()>)
        })?;

        let hash = tx_hash(&tx);

        let mut guard = pool.lock().await;
        match guard.insert_tx(hash.clone(), tx) {
            InsertionOutcome::Inserted(h) => Ok(format!("0x{}", h)),
            InsertionOutcome::AlreadyKnown(h) => Ok(format!("0x{}", h)),
            InsertionOutcome::Rejected(reason) => Err(map_reject(reason)),
        }
    })?;

    module.register_async_method(
        "pool_size",
        |_, pool, _| async move { pool.lock().await.len() },
    )?;
    let handle = server.start(module);
    Ok(handle)
}

fn map_reject(reason: RejectReason) -> ErrorObjectOwned {
    match reason {
        RejectReason::PoolFull => {
            ErrorObjectOwned::owned(ErrorCode::ServerError(1).code(), "txpool full", None::<()>)
        }
        RejectReason::FeeTooLow { min_fee, got } => ErrorObjectOwned::owned(
            ErrorCode::InvalidParams.code(),
            format!("fee too low: got {got}, need >= {min_fee}"),
            None::<()>,
        ),
    }
}
