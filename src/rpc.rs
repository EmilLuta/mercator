use reqwest::blocking::Client;
use serde::{Deserialize, Serialize};
use thiserror::Error;

pub trait RpcClient {
    fn eth_call(&self, to: &str, data: &str) -> Result<String, RpcError>;
}

#[derive(Debug, Clone, Error)]
pub enum RpcError {
    #[error("rpc transport error: {0}")]
    Transport(String),
    #[error("rpc returned error: {0}")]
    Rpc(String),
    #[error("invalid rpc response: {0}")]
    InvalidResponse(String),
}

pub struct HttpRpcClient {
    url: String,
    client: Client,
}

impl HttpRpcClient {
    pub fn new(url: String, timeout_secs: u64) -> Result<Self, RpcError> {
        let client = Client::builder()
            .timeout(std::time::Duration::from_secs(timeout_secs))
            .build()
            .map_err(|err| RpcError::Transport(err.to_string()))?;

        Ok(Self { url, client })
    }
}

impl RpcClient for HttpRpcClient {
    fn eth_call(&self, to: &str, data: &str) -> Result<String, RpcError> {
        let payload = JsonRpcRequest {
            jsonrpc: "2.0",
            id: 1u64,
            method: "eth_call",
            params: [
                serde_json::json!({
                    "to": to,
                    "data": data,
                }),
                serde_json::json!("latest"),
            ],
        };

        let response = self
            .client
            .post(&self.url)
            .json(&payload)
            .send()
            .map_err(|err| RpcError::Transport(err.to_string()))?;

        let body = response
            .json::<JsonRpcResponse>()
            .map_err(|err| RpcError::InvalidResponse(err.to_string()))?;

        if let Some(error) = body.error {
            return Err(RpcError::Rpc(error.message));
        }

        let result = body
            .result
            .ok_or_else(|| RpcError::InvalidResponse("missing result field".to_string()))?;

        if !result.starts_with("0x") {
            return Err(RpcError::InvalidResponse(
                "eth_call result was not 0x-prefixed".to_string(),
            ));
        }

        Ok(result)
    }
}

#[derive(Debug, Serialize)]
struct JsonRpcRequest<'a> {
    jsonrpc: &'a str,
    id: u64,
    method: &'a str,
    params: [serde_json::Value; 2],
}

#[derive(Debug, Deserialize)]
struct JsonRpcResponse {
    result: Option<String>,
    error: Option<JsonRpcErrorObject>,
}

#[derive(Debug, Deserialize)]
struct JsonRpcErrorObject {
    message: String,
}
