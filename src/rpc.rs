use std::{str::FromStr, time::Duration};

use alloy_primitives::{Address, Bytes};
use alloy_provider::{Provider, ProviderBuilder, network::TransactionBuilder};
use alloy_rpc_types_eth::TransactionRequest;
use thiserror::Error;
use tokio::runtime::Runtime;

pub trait RpcClient {
    fn eth_call(&self, to: &str, data: &str) -> Result<String, RpcError>;
}

#[derive(Debug, Clone, Error)]
pub enum RpcError {
    #[error("rpc transport error: {0}")]
    Transport(String),
    #[error("invalid rpc response: {0}")]
    InvalidResponse(String),
}

pub struct HttpRpcClient {
    rpc_url: reqwest::Url,
    reqwest_client: reqwest::Client,
    runtime: Runtime,
}

impl HttpRpcClient {
    pub fn new(url: String, timeout_secs: u64) -> Result<Self, RpcError> {
        let rpc_url =
            reqwest::Url::parse(&url).map_err(|err| RpcError::InvalidResponse(err.to_string()))?;

        let reqwest_client = reqwest::Client::builder()
            .timeout(Duration::from_secs(timeout_secs))
            .build()
            .map_err(|err| RpcError::Transport(err.to_string()))?;

        let runtime = tokio::runtime::Builder::new_multi_thread()
            .enable_all()
            .build()
            .map_err(|err| RpcError::Transport(err.to_string()))?;

        Ok(Self {
            rpc_url,
            reqwest_client,
            runtime,
        })
    }
}

impl RpcClient for HttpRpcClient {
    fn eth_call(&self, to: &str, data: &str) -> Result<String, RpcError> {
        let to_address =
            Address::from_str(to).map_err(|err| RpcError::InvalidResponse(err.to_string()))?;
        let calldata =
            Bytes::from_str(data).map_err(|err| RpcError::InvalidResponse(err.to_string()))?;

        let tx = TransactionRequest::default()
            .with_to(to_address)
            .with_input(calldata);

        let provider = ProviderBuilder::new()
            .connect_reqwest(self.reqwest_client.clone(), self.rpc_url.clone());

        let result = self
            .runtime
            .block_on(async { provider.call(tx).await })
            .map_err(|err| RpcError::Transport(err.to_string()))?;

        Ok(result.to_string())
    }
}
