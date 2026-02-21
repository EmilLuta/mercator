use crate::model::ScanSnapshot;
use std::collections::BTreeMap;

pub fn render_snapshot(snapshot: &ScanSnapshot, verbose: bool) -> String {
    let mut ctm_chain_counts = BTreeMap::new();
    for entry in &snapshot.chain_ctms {
        let count = ctm_chain_counts.entry(entry.ctm.as_str()).or_insert(0usize);
        *count += 1;
    }

    let mut lines = vec![
        format!("Bridgehub: {}", snapshot.bridgehub),
        format!("CTMs discovered: {}", snapshot.ctms.len()),
        String::new(),
        "CTMs".to_string(),
    ];

    if snapshot.ctms.is_empty() {
        lines.push("  - none resolved".to_string());
    } else {
        for ctm in &snapshot.ctms {
            let chain_count = ctm_chain_counts.get(ctm.as_str()).copied().unwrap_or(0);
            lines.push(format!("  - {ctm} ({chain_count} chains)"));
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::{ChainCtm, ScanSnapshot};

    #[test]
    fn renders_basic_snapshot() {
        let snapshot = ScanSnapshot {
            bridgehub: "0x0000000000000000000000000000000000000001".to_string(),
            chain_ids: vec![324],
            chain_ctms: vec![ChainCtm {
                chain_id: 324,
                ctm: "0x0000000000000000000000000000000000000002".to_string(),
            }],
            ctms: vec!["0x0000000000000000000000000000000000000002".to_string()],
            warnings: vec![],
        };

        let output = render_snapshot(&snapshot, false);
        assert!(output.contains("CTMs discovered: 1"));
        assert!(output.contains("0x0000000000000000000000000000000000000002 (1 chains)"));
        assert!(!output.contains("Chain -> CTM"));
    }
}
