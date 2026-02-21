use thiserror::Error;

use crate::rpc::{RpcClient, RpcError};

pub const GET_ALL_ZK_CHAIN_CHAIN_IDS_SELECTOR: &str = "0x68b8d331";
const CHAIN_TYPE_MANAGER_SELECTOR: &str = "0x9d5bd3da";

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
    let response = client.eth_call(bridgehub, GET_ALL_ZK_CHAIN_CHAIN_IDS_SELECTOR)?;
    decode_uint256_array(&response)
}

pub fn get_chain_type_manager(
    client: &dyn RpcClient,
    bridgehub: &str,
    chain_id: u64,
) -> Result<String, BridgehubError> {
    let calldata = encode_chain_type_manager_calldata(chain_id);
    let response = client.eth_call(bridgehub, &calldata)?;
    decode_address_word(&response)
}

pub fn encode_chain_type_manager_calldata(chain_id: u64) -> String {
    format!("{CHAIN_TYPE_MANAGER_SELECTOR}{chain_id:064x}")
}

fn decode_uint256_array(data: &str) -> Result<Vec<u64>, BridgehubError> {
    let bytes = decode_hex_data(data)?;

    if bytes.len() < 64 {
        return Err(BridgehubError::Decode(
            "uint256[] response is too short".to_string(),
        ));
    }

    let offset = read_usize_word(&bytes, 0)?;
    if offset + 32 > bytes.len() {
        return Err(BridgehubError::Decode(
            "array offset points outside response".to_string(),
        ));
    }

    let len = read_usize_word(&bytes, offset)?;
    let mut values = Vec::with_capacity(len);
    let mut cursor = offset + 32;

    for _ in 0..len {
        if cursor + 32 > bytes.len() {
            return Err(BridgehubError::Decode(
                "array item points outside response".to_string(),
            ));
        }
        values.push(read_u64_word(&bytes[cursor..cursor + 32])?);
        cursor += 32;
    }

    Ok(values)
}

fn decode_address_word(data: &str) -> Result<String, BridgehubError> {
    let bytes = decode_hex_data(data)?;
    if bytes.len() != 32 {
        return Err(BridgehubError::Decode(format!(
            "address response must be 32 bytes, got {}",
            bytes.len()
        )));
    }

    let address = &bytes[12..32];
    Ok(format!("0x{}", hex::encode(address)))
}

fn decode_hex_data(value: &str) -> Result<Vec<u8>, BridgehubError> {
    let stripped = value.strip_prefix("0x").ok_or_else(|| {
        BridgehubError::Decode("eth_call result was not 0x-prefixed".to_string())
    })?;

    hex::decode(stripped).map_err(|err| BridgehubError::Decode(err.to_string()))
}

fn read_usize_word(bytes: &[u8], offset: usize) -> Result<usize, BridgehubError> {
    if offset + 32 > bytes.len() {
        return Err(BridgehubError::Decode(
            "attempted to read outside response bounds".to_string(),
        ));
    }

    let word = &bytes[offset..offset + 32];
    if word[..24].iter().any(|byte| *byte != 0) {
        return Err(BridgehubError::Decode(
            "value does not fit into u64".to_string(),
        ));
    }

    let mut low = [0u8; 8];
    low.copy_from_slice(&word[24..32]);
    let value_u64 = u64::from_be_bytes(low);
    usize::try_from(value_u64).map_err(|_| BridgehubError::Decode("usize conversion failed".to_string()))
}

fn read_u64_word(word: &[u8]) -> Result<u64, BridgehubError> {
    if word.len() != 32 {
        return Err(BridgehubError::Decode("word must be 32 bytes".to_string()));
    }

    if word[..24].iter().any(|byte| *byte != 0) {
        return Err(BridgehubError::Decode(
            "value does not fit into u64".to_string(),
        ));
    }

    let mut low = [0u8; 8];
    low.copy_from_slice(&word[24..32]);
    Ok(u64::from_be_bytes(low))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn encodes_chain_type_manager_calldata() {
        let data = encode_chain_type_manager_calldata(324);
        assert_eq!(
            data,
            "0x9d5bd3da0000000000000000000000000000000000000000000000000000000000000144"
        );
    }

    #[test]
    fn decodes_uint256_array() {
        let data = "0x0000000000000000000000000000000000000000000000000000000000000020\
                    0000000000000000000000000000000000000000000000000000000000000003\
                    0000000000000000000000000000000000000000000000000000000000000001\
                    0000000000000000000000000000000000000000000000000000000000000144\
                    0000000000000000000000000000000000000000000000000000000000000145";
        let values = decode_uint256_array(data).expect("decode should succeed");
        assert_eq!(values, vec![1, 324, 325]);
    }

    #[test]
    fn decodes_address_word() {
        let data =
            "0x000000000000000000000000aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa";
        let address = decode_address_word(data).expect("decode should succeed");
        assert_eq!(address, "0xaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa");
    }
}
