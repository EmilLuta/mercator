use std::collections::HashMap;

use mercator::{
    rpc::{RpcClient, RpcError},
    scanner::scan_bridgehub_ctms,
};

#[derive(Default)]
struct ScriptedRpc {
    responses: HashMap<String, Result<String, RpcError>>,
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
    let protocol_version_data = "0x2ae9c600";
    let chain_324_zk_chain_data =
        "0xe680c4c10000000000000000000000000000000000000000000000000000000000000144";
    let chain_325_zk_chain_data =
        "0xe680c4c10000000000000000000000000000000000000000000000000000000000000145";
    let chain_326_zk_chain_data =
        "0xe680c4c10000000000000000000000000000000000000000000000000000000000000146";
    let get_verifier_data = "0x46657fe9";
    let chain_324_admin_data =
        "0x301e77650000000000000000000000000000000000000000000000000000000000000144";
    let chain_325_admin_data =
        "0x301e77650000000000000000000000000000000000000000000000000000000000000145";
    let chain_326_admin_data =
        "0x301e77650000000000000000000000000000000000000000000000000000000000000146";
    let chain_324_protocol_data =
        "0xba2389470000000000000000000000000000000000000000000000000000000000000144";
    let chain_325_protocol_data =
        "0xba2389470000000000000000000000000000000000000000000000000000000000000145";
    let chain_326_protocol_data =
        "0xba2389470000000000000000000000000000000000000000000000000000000000000146";

    let rpc = ScriptedRpc::default()
        .with_response(
            chain_ids_data,
            Ok(
                "0x0000000000000000000000000000000000000000000000000000000000000020\
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
        )
        .with_response(
            chain_324_zk_chain_data,
            Ok("0x000000000000000000000000cccccccccccccccccccccccccccccccccccccccc".to_string()),
        )
        .with_response(
            chain_325_zk_chain_data,
            Ok("0x000000000000000000000000dddddddddddddddddddddddddddddddddddddddd".to_string()),
        )
        .with_response(
            chain_326_zk_chain_data,
            Ok("0x000000000000000000000000eeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeee".to_string()),
        )
        .with_response(
            get_verifier_data,
            Ok("0x000000000000000000000000abababababababababababababababababababab".to_string()),
        )
        .with_response(
            chain_324_admin_data,
            Ok("0x0000000000000000000000009999999999999999999999999999999999999999".to_string()),
        )
        .with_response(
            chain_325_admin_data,
            Ok("0x0000000000000000000000009999999999999999999999999999999999999999".to_string()),
        )
        .with_response(
            chain_326_admin_data,
            Ok("0x0000000000000000000000009999999999999999999999999999999999999999".to_string()),
        )
        .with_response(
            chain_324_protocol_data,
            Ok("0x000000000000000000000000000000000000000000000000000000000000002a".to_string()),
        )
        .with_response(
            chain_325_protocol_data,
            Ok("0x000000000000000000000000000000000000000000000000000000000000002a".to_string()),
        )
        .with_response(
            chain_326_protocol_data,
            Ok("0x000000000000000000000000000000000000000000000000000000000000002a".to_string()),
        )
        .with_response(
            protocol_version_data,
            Ok("0x000000000000000000000000000000000000000000000000000000000000002a".to_string()),
        );

    let snapshot = scan_bridgehub_ctms(&rpc, bridgehub).expect("scan should succeed");

    assert_eq!(snapshot.chain_ids, vec![324, 325, 326]);
    assert_eq!(snapshot.chain_ctms.len(), 3);
    assert_eq!(snapshot.chains.len(), 3);
    assert_eq!(snapshot.ctms.len(), 2);
    assert!(snapshot.warnings.is_empty());
    assert_eq!(
        snapshot.ctms[0].address,
        "0xaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa"
    );
    assert_eq!(snapshot.ctms[0].protocol_version.as_deref(), Some("0.0.42"));
    assert_eq!(
        snapshot.ctms[1].address,
        "0xbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb"
    );
    assert_eq!(snapshot.ctms[1].protocol_version.as_deref(), Some("0.0.42"));
    assert_eq!(
        snapshot.chains[0].chain_contract.as_deref(),
        Some("0xcccccccccccccccccccccccccccccccccccccccc")
    );
    assert_eq!(
        snapshot.chains[0].admin.as_deref(),
        Some("0x9999999999999999999999999999999999999999")
    );
    assert_eq!(
        snapshot.chains[0].verifier.as_deref(),
        Some("0xabababababababababababababababababababab")
    );
    assert_eq!(
        snapshot.chains[0].protocol_version.as_deref(),
        Some("0.0.42")
    );
}
