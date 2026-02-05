use anyhow::{Result, anyhow};
use serde::Deserialize;
use std::net::SocketAddr;

#[derive(Debug, Clone, Deserialize)]
pub struct AppConfig {
    pub rpc: RpcConfig,
}

#[derive(Debug, Clone, Deserialize)]

pub struct RpcConfig {
    pub addr: SocketAddr,
}

impl AppConfig {
    pub fn validate(&self) -> Result<()> {
        if self.rpc.addr.port() == 0 {
            return Err(anyhow!("rpc.addr port must be non-zero"));
        }
        Ok(())
    }
}
