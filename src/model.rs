#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ChainCtm {
    pub chain_id: u64,
    pub ctm: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CtmSummary {
    pub address: String,
    pub protocol_version: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ChainSummary {
    pub chain_id: u64,
    pub ctm: Option<String>,
    pub validator_timelock: Option<String>,
    pub chain_contract: Option<String>,
    pub admin: Option<String>,
    pub admin_owner: Option<String>,
    pub protocol_version: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TopologySnapshot {
    pub bridgehub: String,
    pub chain_ids: Vec<u64>,
    pub chain_ctms: Vec<ChainCtm>,
    pub ctms: Vec<CtmSummary>,
    pub warnings: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ChainInspection {
    pub bridgehub: String,
    pub chain: ChainSummary,
    pub warnings: Vec<String>,
}
