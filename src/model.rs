#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ChainCtm {
    pub chain_id: u64,
    pub ctm: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ScanSnapshot {
    pub bridgehub: String,
    pub chain_ids: Vec<u64>,
    pub chain_ctms: Vec<ChainCtm>,
    pub ctms: Vec<String>,
    pub warnings: Vec<String>,
}
