use std::collections::BTreeSet;

use thiserror::Error;

use crate::model::{ChainCtm, ChainInspection, ChainSummary, CtmSummary, TopologySnapshot};
use crate::rpc::RpcClient;

pub mod bridgehub;

#[derive(Debug, Error)]
pub enum ScanError {
    #[error("bridgehub scan failed: {0}")]
    Bridgehub(#[from] bridgehub::BridgehubError),
}

pub fn scan_bridgehub_topology(
    client: &dyn RpcClient,
    bridgehub: &str,
) -> Result<TopologySnapshot, ScanError> {
    let chain_ids = bridgehub::get_all_zk_chain_chain_ids(client, bridgehub)?;
    let mut chain_ctms = Vec::with_capacity(chain_ids.len());
    let mut warnings = Vec::new();

    for chain_id in &chain_ids {
        match bridgehub::get_chain_type_manager(client, bridgehub, *chain_id) {
            Ok(ctm) => {
                if is_zero_address(&ctm) {
                    warnings.push(format!(
                        "chain {chain_id} returned zero address for chainTypeManager"
                    ));
                } else {
                    chain_ctms.push(ChainCtm {
                        chain_id: *chain_id,
                        ctm,
                    });
                }
            }
            Err(err) => warnings.push(format!(
                "failed to resolve chainTypeManager for chain {chain_id}: {err}"
            )),
        }
    }

    let mut deduped_ctms = BTreeSet::new();
    for mapping in &chain_ctms {
        deduped_ctms.insert(mapping.ctm.clone());
    }

    let mut ctms = Vec::with_capacity(deduped_ctms.len());
    for ctm in deduped_ctms {
        let protocol_version = match bridgehub::get_ctm_protocol_semver(client, &ctm) {
            Ok(version) => Some(version),
            Err(err) => {
                warnings.push(format!(
                    "failed to resolve protocol semver for ctm {ctm}: {err}"
                ));
                None
            }
        };
        ctms.push(CtmSummary {
            address: ctm,
            protocol_version,
        });
    }

    Ok(TopologySnapshot {
        bridgehub: bridgehub.to_string(),
        chain_ids,
        chain_ctms,
        ctms,
        warnings,
    })
}

pub fn inspect_bridgehub_chain(
    client: &dyn RpcClient,
    bridgehub: &str,
    chain_id: u64,
) -> Result<ChainInspection, ScanError> {
    let mut warnings = Vec::new();

    let ctm = match bridgehub::get_chain_type_manager(client, bridgehub, chain_id) {
        Ok(address) if !is_zero_address(&address) => Some(address),
        Ok(_) => {
            warnings.push(format!(
                "chain {chain_id} returned zero address for chainTypeManager"
            ));
            None
        }
        Err(err) => {
            warnings.push(format!(
                "failed to resolve chainTypeManager for chain {chain_id}: {err}"
            ));
            None
        }
    };

    let chain_contract = match bridgehub::get_zk_chain(client, bridgehub, chain_id) {
        Ok(address) if !is_zero_address(&address) => Some(address),
        Ok(_) => None,
        Err(err) => {
            warnings.push(format!(
                "failed to resolve getZKChain for chain {chain_id}: {err}"
            ));
            None
        }
    };

    let verifier = match chain_contract.as_deref() {
        Some(chain_contract) => match bridgehub::get_chain_verifier(client, chain_contract) {
            Ok(address) if !is_zero_address(&address) => Some(address),
            Ok(_) => None,
            Err(err) => {
                warnings.push(format!(
                    "failed to resolve getVerifier for chain {chain_id}: {err}"
                ));
                None
            }
        },
        None => None,
    };

    let admin = match ctm.as_deref() {
        Some(ctm) => match bridgehub::get_ctm_chain_admin(client, ctm, chain_id) {
            Ok(address) if !is_zero_address(&address) => Some(address),
            Ok(_) => None,
            Err(err) => {
                warnings.push(format!(
                    "failed to resolve getChainAdmin for chain {chain_id}: {err}"
                ));
                None
            }
        },
        None => None,
    };

    let protocol_version = match ctm.as_deref() {
        Some(ctm) => match bridgehub::get_ctm_chain_protocol_semver(client, ctm, chain_id) {
            Ok(version) => Some(version),
            Err(err) => {
                warnings.push(format!(
                    "failed to resolve getProtocolVersion for chain {chain_id}: {err}"
                ));
                None
            }
        },
        None => None,
    };

    Ok(ChainInspection {
        bridgehub: bridgehub.to_string(),
        chain: ChainSummary {
            chain_id,
            ctm,
            chain_contract,
            verifier,
            admin,
            protocol_version,
        },
        warnings,
    })
}

fn is_zero_address(address: &str) -> bool {
    address == "0x0000000000000000000000000000000000000000"
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use crate::rpc::{RpcClient, RpcError};

    use super::*;

    #[derive(Default)]
    struct MockRpcClient {
        responses: HashMap<String, Result<String, RpcError>>,
    }

    impl MockRpcClient {
        fn with_response(mut self, data: &str, response: Result<String, RpcError>) -> Self {
            self.responses.insert(data.to_string(), response);
            self
        }
    }

    impl RpcClient for MockRpcClient {
        fn eth_call(&self, _to: &str, data: &str) -> Result<String, RpcError> {
            self.responses.get(data).cloned().unwrap_or_else(|| {
                Err(RpcError::InvalidResponse(format!(
                    "missing mock response for calldata: {data}"
                )))
            })
        }
    }

    #[test]
    fn topology_scanner_collects_and_dedupes_ctms() {
        let chain_ids_data = bridgehub::encode_get_all_zk_chain_chain_ids_calldata();
        let chain_324_data = bridgehub::encode_chain_type_manager_calldata(324);
        let chain_325_data = bridgehub::encode_chain_type_manager_calldata(325);
        let protocol_version_data = bridgehub::encode_protocol_version_calldata();

        let mock = MockRpcClient::default()
            .with_response(
                &chain_ids_data,
                Ok("0x0000000000000000000000000000000000000000000000000000000000000020000000000000000000000000000000000000000000000000000000000000000200000000000000000000000000000000000000000000000000000000000001440000000000000000000000000000000000000000000000000000000000000145".to_string()),
            )
            .with_response(
                &chain_324_data,
                Ok("0x000000000000000000000000aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa".to_string()),
            )
            .with_response(
                &chain_325_data,
                Ok("0x000000000000000000000000aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa".to_string()),
            )
            .with_response(
                &protocol_version_data,
                Ok("0x0000000000000000000000000000000000000000000000000000000000000007".to_string()),
            );

        let snapshot = scan_bridgehub_topology(&mock, "0x0000000000000000000000000000000000000001")
            .expect("scan should succeed");

        assert_eq!(snapshot.chain_ids, vec![324, 325]);
        assert_eq!(snapshot.chain_ctms.len(), 2);
        assert_eq!(snapshot.ctms.len(), 1);
        assert!(snapshot.warnings.is_empty());
        assert_eq!(
            snapshot.ctms[0].address,
            "0xaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa"
        );
        assert_eq!(snapshot.ctms[0].protocol_version, Some("0.0.7".to_string()));
    }

    #[test]
    fn inspect_chain_resolves_deep_details() {
        let chain_324_data = bridgehub::encode_chain_type_manager_calldata(324);
        let chain_324_zk_chain_data = bridgehub::encode_get_zk_chain_calldata(324);
        let get_verifier_data = bridgehub::encode_get_verifier_calldata();
        let chain_324_admin_data = bridgehub::encode_get_chain_admin_calldata(324);
        let chain_324_protocol_data = bridgehub::encode_get_chain_protocol_version_calldata(324);

        let mock = MockRpcClient::default()
            .with_response(
                &chain_324_data,
                Ok(
                    "0x000000000000000000000000aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa"
                        .to_string(),
                ),
            )
            .with_response(
                &chain_324_zk_chain_data,
                Ok(
                    "0x000000000000000000000000cccccccccccccccccccccccccccccccccccccccc"
                        .to_string(),
                ),
            )
            .with_response(
                &get_verifier_data,
                Ok(
                    "0x000000000000000000000000ffffffffffffffffffffffffffffffffffffffff"
                        .to_string(),
                ),
            )
            .with_response(
                &chain_324_admin_data,
                Ok(
                    "0x000000000000000000000000eeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeee"
                        .to_string(),
                ),
            )
            .with_response(
                &chain_324_protocol_data,
                Ok(
                    "0x0000000000000000000000000000000000000000000000000000000000000007"
                        .to_string(),
                ),
            );

        let inspection =
            inspect_bridgehub_chain(&mock, "0x0000000000000000000000000000000000000001", 324)
                .expect("inspect should succeed");

        assert_eq!(inspection.chain.chain_id, 324);
        assert_eq!(
            inspection.chain.ctm.as_deref(),
            Some("0xaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa")
        );
        assert_eq!(
            inspection.chain.chain_contract.as_deref(),
            Some("0xcccccccccccccccccccccccccccccccccccccccc")
        );
        assert_eq!(
            inspection.chain.verifier.as_deref(),
            Some("0xffffffffffffffffffffffffffffffffffffffff")
        );
        assert_eq!(
            inspection.chain.admin.as_deref(),
            Some("0xeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeee")
        );
        assert_eq!(inspection.chain.protocol_version.as_deref(), Some("0.0.7"));
        assert!(inspection.warnings.is_empty());
    }
}
