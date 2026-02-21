use std::collections::BTreeMap;

use crate::model::{ChainInspection, TopologySnapshot};

pub fn render_topology(snapshot: &TopologySnapshot, verbose: bool) -> String {
    let mut ctm_chain_ids: BTreeMap<&str, Vec<u64>> = BTreeMap::new();
    for entry in &snapshot.chain_ctms {
        ctm_chain_ids
            .entry(entry.ctm.as_str())
            .or_default()
            .push(entry.chain_id);
    }
    for chain_ids in ctm_chain_ids.values_mut() {
        chain_ids.sort_unstable();
    }

    let mut lines = vec![
        format!("Bridgehub: {}", snapshot.bridgehub),
        format!("Chains discovered: {}", snapshot.chain_ids.len()),
        format!("CTMs discovered: {}", snapshot.ctms.len()),
        String::new(),
        "CTMs".to_string(),
    ];

    if snapshot.ctms.is_empty() {
        lines.push("  - none resolved".to_string());
    } else {
        for ctm in &snapshot.ctms {
            let chain_ids = ctm_chain_ids
                .get(ctm.address.as_str())
                .cloned()
                .unwrap_or_default();
            let chain_count = chain_ids.len();
            let chain_ids_text = if chain_ids.is_empty() {
                "none".to_string()
            } else {
                chain_ids
                    .iter()
                    .map(u64::to_string)
                    .collect::<Vec<_>>()
                    .join(",")
            };
            let protocol_version = ctm.protocol_version.as_deref().unwrap_or("unknown");
            lines.push(format!(
                "  - {} (protocol version: {protocol_version}, chain count: {chain_count}, chains: {chain_ids_text})",
                ctm.address
            ));
        }
    }

    if !snapshot.warnings.is_empty() {
        lines.push(String::new());
        lines.push("Warnings".to_string());
        for warning in &snapshot.warnings {
            lines.push(format!("  - {warning}"));
        }
    } else if verbose {
        lines.push(String::new());
        lines.push("Warnings".to_string());
        lines.push("  - none".to_string());
    }

    lines.join("\n")
}

pub fn render_chain_inspection(inspection: &ChainInspection, verbose: bool) -> String {
    let chain = &inspection.chain;
    let ctm = chain.ctm.as_deref().unwrap_or("unknown");
    let validator_timelock = chain.validator_timelock.as_deref().unwrap_or("unknown");
    let diamond = chain.chain_contract.as_deref().unwrap_or("unknown");
    let admin = chain.admin.as_deref().unwrap_or("unknown");
    let admin_owner = chain.admin_owner.as_deref().unwrap_or("unknown");
    let protocol = chain.protocol_version.as_deref().unwrap_or("unknown");

    let mut lines = vec![
        format!("Bridgehub: {}", inspection.bridgehub),
        format!("Chain ID: {}", chain.chain_id),
        String::new(),
        "Details".to_string(),
        format!("  - CTM: {ctm}"),
        format!("  - ValidatorTimelock: {validator_timelock}"),
        format!("  - Diamond: {diamond}"),
        format!("  - Protocol: {protocol}"),
        format!("  - Admin: {admin} (owner: {admin_owner})"),
    ];

    if !inspection.warnings.is_empty() {
        lines.push(String::new());
        lines.push("Warnings".to_string());
        for warning in &inspection.warnings {
            lines.push(format!("  - {warning}"));
        }
    } else if verbose {
        lines.push(String::new());
        lines.push("Warnings".to_string());
        lines.push("  - none".to_string());
    }

    lines.join("\n")
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::{ChainCtm, ChainInspection, ChainSummary, CtmSummary, TopologySnapshot};

    #[test]
    fn renders_topology_snapshot() {
        let snapshot = TopologySnapshot {
            bridgehub: "0x0000000000000000000000000000000000000001".to_string(),
            chain_ids: vec![324, 325],
            chain_ctms: vec![
                ChainCtm {
                    chain_id: 325,
                    ctm: "0x0000000000000000000000000000000000000002".to_string(),
                },
                ChainCtm {
                    chain_id: 324,
                    ctm: "0x0000000000000000000000000000000000000002".to_string(),
                },
            ],
            ctms: vec![CtmSummary {
                address: "0x0000000000000000000000000000000000000002".to_string(),
                protocol_version: Some("17".to_string()),
            }],
            warnings: vec![],
        };

        let output = render_topology(&snapshot, false);
        assert!(output.contains("Chains discovered: 2"));
        assert!(output.contains("CTMs discovered: 1"));
        assert!(output.contains(
            "0x0000000000000000000000000000000000000002 (protocol version: 17, chain count: 2, chains: 324,325)"
        ));
        assert!(!output.contains("Details"));
    }

    #[test]
    fn renders_chain_inspection() {
        let inspection = ChainInspection {
            bridgehub: "0x0000000000000000000000000000000000000001".to_string(),
            chain: ChainSummary {
                chain_id: 324,
                ctm: Some("0x0000000000000000000000000000000000000002".to_string()),
                validator_timelock: Some("0x0000000000000000000000000000000000000006".to_string()),
                chain_contract: Some("0x0000000000000000000000000000000000000003".to_string()),
                admin: Some("0x0000000000000000000000000000000000000004".to_string()),
                admin_owner: Some("0x0000000000000000000000000000000000000007".to_string()),
                protocol_version: Some("17.0.0".to_string()),
            },
            warnings: vec![],
        };

        let output = render_chain_inspection(&inspection, false);
        assert!(output.contains("Chain ID: 324"));
        assert!(output.contains("Details"));
        assert!(output.contains("CTM: 0x0000000000000000000000000000000000000002"));
        assert!(output.contains("ValidatorTimelock: 0x0000000000000000000000000000000000000006"));
        assert!(output
            .contains("Admin: 0x0000000000000000000000000000000000000004 (owner: 0x0000000000000000000000000000000000000007)"));
        assert!(!output.contains("Verifier:"));
    }
}
