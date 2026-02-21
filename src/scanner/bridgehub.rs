use alloy_primitives::U256;
use alloy_sol_types::{SolCall, sol};
use thiserror::Error;

use crate::rpc::{RpcClient, RpcError};

sol! {
    function getAllZKChainChainIDs() external view returns (uint256[] chainIds);
    function chainTypeManager(uint256 chainId) external view returns (address ctm);
    function getZKChain(uint256 chainId) external view returns (address chainContract);
    function protocolVersion() external view returns (uint256 version);
    function getSemverProtocolVersion() external view returns (uint32 major, uint32 minor, uint32 patch);
    function getChainAdmin(uint256 chainId) external view returns (address admin);
    function getProtocolVersion(uint256 chainId) external view returns (uint256 version);
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

pub fn get_zk_chain(
    client: &dyn RpcClient,
    bridgehub: &str,
    chain_id: u64,
) -> Result<String, BridgehubError> {
    let calldata = encode_get_zk_chain_calldata(chain_id);
    let response = client.eth_call(bridgehub, &calldata)?;
    let bytes = decode_hex_data(&response)?;
    let decoded = getZKChainCall::abi_decode_returns(&bytes)
        .map_err(|err| BridgehubError::Decode(err.to_string()))?;
    Ok(format!("{decoded:#x}"))
}

pub fn get_ctm_protocol_semver(
    client: &dyn RpcClient,
    ctm: &str,
) -> Result<String, BridgehubError> {
    if let Ok((major, minor, patch)) = get_ctm_semver_components(client, ctm) {
        return Ok(format!("{major}.{minor}.{patch}"));
    }

    let raw = get_ctm_protocol_version_raw(client, ctm)?;
    let (major, minor, patch) = decode_packed_semver(raw)?;
    Ok(format!("{major}.{minor}.{patch}"))
}

pub fn get_ctm_chain_admin(
    client: &dyn RpcClient,
    ctm: &str,
    chain_id: u64,
) -> Result<String, BridgehubError> {
    let calldata = encode_get_chain_admin_calldata(chain_id);
    let response = client.eth_call(ctm, &calldata)?;
    let bytes = decode_hex_data(&response)?;
    let decoded = getChainAdminCall::abi_decode_returns(&bytes)
        .map_err(|err| BridgehubError::Decode(err.to_string()))?;
    Ok(format!("{decoded:#x}"))
}

pub fn get_ctm_chain_protocol_semver(
    client: &dyn RpcClient,
    ctm: &str,
    chain_id: u64,
) -> Result<String, BridgehubError> {
    let raw = get_ctm_chain_protocol_version_raw(client, ctm, chain_id)?;
    let (major, minor, patch) = decode_packed_semver(raw)?;
    Ok(format!("{major}.{minor}.{patch}"))
}

fn get_ctm_semver_components(
    client: &dyn RpcClient,
    ctm: &str,
) -> Result<(u32, u32, u32), BridgehubError> {
    let calldata = encode_get_semver_protocol_version_calldata();
    let response = client.eth_call(ctm, &calldata)?;
    let bytes = decode_hex_data(&response)?;
    let decoded = getSemverProtocolVersionCall::abi_decode_returns(&bytes)
        .map_err(|err| BridgehubError::Decode(err.to_string()))?;
    Ok((decoded.major, decoded.minor, decoded.patch))
}

fn get_ctm_protocol_version_raw(client: &dyn RpcClient, ctm: &str) -> Result<U256, BridgehubError> {
    let calldata = encode_protocol_version_calldata();
    let response = client.eth_call(ctm, &calldata)?;
    let bytes = decode_hex_data(&response)?;
    let decoded = protocolVersionCall::abi_decode_returns(&bytes)
        .map_err(|err| BridgehubError::Decode(err.to_string()))?;
    Ok(decoded)
}

fn get_ctm_chain_protocol_version_raw(
    client: &dyn RpcClient,
    ctm: &str,
    chain_id: u64,
) -> Result<U256, BridgehubError> {
    let calldata = encode_get_chain_protocol_version_calldata(chain_id);
    let response = client.eth_call(ctm, &calldata)?;
    let bytes = decode_hex_data(&response)?;
    let decoded = getProtocolVersionCall::abi_decode_returns(&bytes)
        .map_err(|err| BridgehubError::Decode(err.to_string()))?;
    Ok(decoded)
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

pub fn encode_get_zk_chain_calldata(chain_id: u64) -> String {
    let calldata = getZKChainCall {
        chainId: U256::from(chain_id),
    }
    .abi_encode();
    format!("0x{}", hex::encode(calldata))
}

pub fn encode_protocol_version_calldata() -> String {
    format!("0x{}", hex::encode(protocolVersionCall {}.abi_encode()))
}

pub fn encode_get_semver_protocol_version_calldata() -> String {
    format!(
        "0x{}",
        hex::encode(getSemverProtocolVersionCall {}.abi_encode())
    )
}

pub fn encode_get_chain_admin_calldata(chain_id: u64) -> String {
    let calldata = getChainAdminCall {
        chainId: U256::from(chain_id),
    }
    .abi_encode();
    format!("0x{}", hex::encode(calldata))
}

pub fn encode_get_chain_protocol_version_calldata(chain_id: u64) -> String {
    let calldata = getProtocolVersionCall {
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

fn decode_packed_semver(value: U256) -> Result<(u32, u32, u32), BridgehubError> {
    let mask = U256::from(u32::MAX as u64);
    let major_u64 = ((value >> 64usize) & mask).to::<u64>();
    let minor_u64 = ((value >> 32usize) & mask).to::<u64>();
    let patch_u64 = (value & mask).to::<u64>();

    let major = u32::try_from(major_u64)
        .map_err(|_| BridgehubError::Decode("semver major does not fit into u32".to_string()))?;
    let minor = u32::try_from(minor_u64)
        .map_err(|_| BridgehubError::Decode("semver minor does not fit into u32".to_string()))?;
    let patch = u32::try_from(patch_u64)
        .map_err(|_| BridgehubError::Decode("semver patch does not fit into u32".to_string()))?;

    Ok((major, minor, patch))
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
    fn encodes_get_zk_chain_calldata() {
        let data = encode_get_zk_chain_calldata(324);
        assert_eq!(
            data,
            "0xe680c4c10000000000000000000000000000000000000000000000000000000000000144"
        );
    }

    #[test]
    fn encodes_protocol_version_calldata() {
        let data = encode_protocol_version_calldata();
        assert_eq!(data, "0x2ae9c600");
    }

    #[test]
    fn encodes_get_semver_protocol_version_calldata() {
        let data = encode_get_semver_protocol_version_calldata();
        assert_eq!(data, "0xf5c1182c");
    }

    #[test]
    fn encodes_get_chain_admin_calldata() {
        let data = encode_get_chain_admin_calldata(324);
        assert_eq!(
            data,
            "0x301e77650000000000000000000000000000000000000000000000000000000000000144"
        );
    }

    #[test]
    fn encodes_get_chain_protocol_version_calldata() {
        let data = encode_get_chain_protocol_version_calldata(324);
        assert_eq!(
            data,
            "0xba2389470000000000000000000000000000000000000000000000000000000000000144"
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

    #[test]
    fn decodes_protocol_version_return() {
        let data = "0x000000000000000000000000000000000000000000000000000000000000000f";
        let bytes = decode_hex_data(data).expect("hex decode should succeed");
        let decoded =
            protocolVersionCall::abi_decode_returns(&bytes).expect("abi decode should work");
        assert_eq!(decoded.to_string(), "15");
    }

    #[test]
    fn decodes_get_semver_protocol_version_return() {
        let data = "0x\
                    0000000000000000000000000000000000000000000000000000000000000001\
                    000000000000000000000000000000000000000000000000000000000000001d\
                    0000000000000000000000000000000000000000000000000000000000000004";
        let bytes = decode_hex_data(data).expect("hex decode should succeed");
        let decoded = getSemverProtocolVersionCall::abi_decode_returns(&bytes)
            .expect("abi decode should work");
        assert_eq!(decoded.major, 1);
        assert_eq!(decoded.minor, 29);
        assert_eq!(decoded.patch, 4);
    }

    #[test]
    fn decodes_packed_semver_value() {
        let packed = (U256::from(1u32) << 64) | (U256::from(29u32) << 32) | U256::from(4u32);
        let decoded = decode_packed_semver(packed).expect("packed decode should succeed");
        assert_eq!(decoded, (1, 29, 4));
    }
}
