use std::collections::BTreeSet;

use thiserror::Error;

use crate::model::{ChainCtm, ScanSnapshot};
use crate::rpc::RpcClient;

pub mod bridgehub;

#[derive(Debug, Error)]
pub enum ScanError {
    #[error("bridgehub scan failed: {0}")]
    Bridgehub(#[from] bridgehub::BridgehubError),
}

pub fn scan_bridgehub_ctms(
    client: &dyn RpcClient,
    bridgehub: &str,
) -> Result<ScanSnapshot, ScanError> {
    let chain_ids = bridgehub::get_all_zk_chain_chain_ids(client, bridgehub)?;
    let mut chain_ctms = Vec::with_capacity(chain_ids.len());
    let mut warnings = Vec::new();

    for chain_id in &chain_ids {
        match bridgehub::get_chain_type_manager(client, bridgehub, *chain_id) {
            Ok(ctm) => {
                if ctm == "0x0000000000000000000000000000000000000000" {
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

    let mut deduped = BTreeSet::new();
    for mapping in &chain_ctms {
        deduped.insert(mapping.ctm.clone());
    }

    Ok(ScanSnapshot {
        bridgehub: bridgehub.to_string(),
        chain_ids,
        chain_ctms,
        ctms: deduped.into_iter().collect(),
        warnings,
    })
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
    fn scanner_collects_and_dedupes_ctms() {
        let chain_ids_data = bridgehub::encode_get_all_zk_chain_chain_ids_calldata();
        let chain_324_data = bridgehub::encode_chain_type_manager_calldata(324);
        let chain_325_data = bridgehub::encode_chain_type_manager_calldata(325);

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
            );

        let snapshot = scan_bridgehub_ctms(&mock, "0x0000000000000000000000000000000000000001")
            .expect("scan should succeed");

        assert_eq!(snapshot.chain_ids, vec![324, 325]);
        assert_eq!(snapshot.chain_ctms.len(), 2);
        assert_eq!(snapshot.ctms.len(), 1);
        assert_eq!(
            snapshot.ctms[0],
            "0xaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa"
        );
    }
}
