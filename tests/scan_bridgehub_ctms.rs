use std::collections::HashMap;

use mercator::{
    rpc::{RpcClient, RpcError},
    scanner::scan_bridgehub_ctms,
};

struct ScriptedRpc {
    responses: HashMap<String, Result<String, RpcError>>,
}

impl Default for ScriptedRpc {
    fn default() -> Self {
        Self {
            responses: HashMap::new(),
        }
    }
}

impl ScriptedRpc {
    fn with_response(mut self, data: &str, response: Result<String, RpcError>) -> Self {
        self.responses.insert(data.to_string(), response);
        self
    }
}

impl RpcClient for ScriptedRpc {
    fn eth_call(&self, _to: &str, data: &str) -> Result<String, RpcError> {
        self.responses.get(data).cloned().unwrap_or_else(|| {
            Err(RpcError::InvalidResponse(format!(
                "missing scripted response for calldata: {data}"
            )))
        })
    }
}

#[test]
fn scan_bridgehub_ctms_with_scripted_rpc() {
    let bridgehub = "0x1111111111111111111111111111111111111111";
    let chain_ids_data = "0x68b8d331";

    let rpc = ScriptedRpc::default()
        .with_response(
            chain_ids_data,
            Ok("0x0000000000000000000000000000000000000000000000000000000000000020\
                0000000000000000000000000000000000000000000000000000000000000003\
                0000000000000000000000000000000000000000000000000000000000000144\
                0000000000000000000000000000000000000000000000000000000000000145\
                0000000000000000000000000000000000000000000000000000000000000146"
                .replace(' ', ""),
            ),
        )
        .with_response(
            "0x9d5bd3da0000000000000000000000000000000000000000000000000000000000000144",
            Ok("0x000000000000000000000000aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa".to_string()),
        )
        .with_response(
            "0x9d5bd3da0000000000000000000000000000000000000000000000000000000000000145",
            Ok("0x000000000000000000000000bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb".to_string()),
        )
        .with_response(
            "0x9d5bd3da0000000000000000000000000000000000000000000000000000000000000146",
            Ok("0x000000000000000000000000aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa".to_string()),
        );

    let snapshot = scan_bridgehub_ctms(&rpc, bridgehub).expect("scan should succeed");

    assert_eq!(snapshot.chain_ids, vec![324, 325, 326]);
    assert_eq!(snapshot.chain_ctms.len(), 3);
    assert_eq!(snapshot.ctms.len(), 2);
    assert!(snapshot.warnings.is_empty());
    assert_eq!(snapshot.ctms[0], "0xaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa");
    assert_eq!(snapshot.ctms[1], "0xbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb");
}
