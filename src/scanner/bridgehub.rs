use alloy_primitives::U256;
use alloy_sol_types::{SolCall, sol};
use thiserror::Error;

use crate::rpc::{RpcClient, RpcError};

sol! {
    function getAllZKChainChainIDs() external view returns (uint256[] chainIds);
    function chainTypeManager(uint256 chainId) external view returns (address ctm);
}

#[derive(Debug, Error)]
pub enum BridgehubError {
    #[error("rpc error: {0}")]
    Rpc(#[from] RpcError),
    #[error("decode error: {0}")]
    Decode(String),
}

pub fn get_all_zk_chain_chain_ids(
    client: &dyn RpcClient,
    bridgehub: &str,
) -> Result<Vec<u64>, BridgehubError> {
    let calldata = encode_get_all_zk_chain_chain_ids_calldata();
    let response = client.eth_call(bridgehub, &calldata)?;
    let bytes = decode_hex_data(&response)?;
    let decoded = getAllZKChainChainIDsCall::abi_decode_returns(&bytes)
        .map_err(|err| BridgehubError::Decode(err.to_string()))?;

    decoded
        .into_iter()
        .map(u256_to_u64)
        .collect::<Result<Vec<_>, _>>()
}

pub fn get_chain_type_manager(
    client: &dyn RpcClient,
    bridgehub: &str,
    chain_id: u64,
) -> Result<String, BridgehubError> {
    let calldata = encode_chain_type_manager_calldata(chain_id);
    let response = client.eth_call(bridgehub, &calldata)?;
    let bytes = decode_hex_data(&response)?;
    let decoded = chainTypeManagerCall::abi_decode_returns(&bytes)
        .map_err(|err| BridgehubError::Decode(err.to_string()))?;
    Ok(format!("{decoded:#x}"))
}

pub fn encode_get_all_zk_chain_chain_ids_calldata() -> String {
    format!(
        "0x{}",
        hex::encode(getAllZKChainChainIDsCall {}.abi_encode())
    )
}

pub fn encode_chain_type_manager_calldata(chain_id: u64) -> String {
    let calldata = chainTypeManagerCall {
        chainId: U256::from(chain_id),
    }
    .abi_encode();
    format!("0x{}", hex::encode(calldata))
}

fn decode_hex_data(value: &str) -> Result<Vec<u8>, BridgehubError> {
    let stripped = value
        .strip_prefix("0x")
        .ok_or_else(|| BridgehubError::Decode("eth_call result was not 0x-prefixed".to_string()))?;

    hex::decode(stripped).map_err(|err| BridgehubError::Decode(err.to_string()))
}

fn u256_to_u64(value: U256) -> Result<u64, BridgehubError> {
    u64::try_from(value)
        .map_err(|_| BridgehubError::Decode("decoded chain id does not fit into u64".to_string()))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn encodes_get_all_chain_ids_calldata() {
        let data = encode_get_all_zk_chain_chain_ids_calldata();
        assert_eq!(data, "0x68b8d331");
    }

    #[test]
    fn encodes_chain_type_manager_calldata() {
        let data = encode_chain_type_manager_calldata(324);
        assert_eq!(
            data,
            "0x9d5bd3da0000000000000000000000000000000000000000000000000000000000000144"
        );
    }

    #[test]
    fn decodes_get_all_chain_ids_return() {
        let data = "0x0000000000000000000000000000000000000000000000000000000000000020\
                    0000000000000000000000000000000000000000000000000000000000000003\
                    0000000000000000000000000000000000000000000000000000000000000001\
                    0000000000000000000000000000000000000000000000000000000000000144\
                    0000000000000000000000000000000000000000000000000000000000000145";
        let bytes = decode_hex_data(data).expect("hex decode should succeed");
        let decoded = getAllZKChainChainIDsCall::abi_decode_returns(&bytes)
            .expect("abi decode should succeed");
        let values = decoded
            .into_iter()
            .map(u256_to_u64)
            .collect::<Result<Vec<_>, _>>()
            .expect("u64 conversion should succeed");
        assert_eq!(values, vec![1, 324, 325]);
    }

    #[test]
    fn decodes_chain_type_manager_return() {
        let data = "0x000000000000000000000000aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa";
        let bytes = decode_hex_data(data).expect("hex decode should succeed");
        let decoded =
            chainTypeManagerCall::abi_decode_returns(&bytes).expect("abi decode should work");
        assert_eq!(
            format!("{decoded:#x}"),
            "0xaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa"
        );
    }
}
