use clap::{Args, Parser, Subcommand};
use std::str::FromStr;

use alloy_primitives::Address;

#[derive(Debug, Parser)]
#[command(
    name = "mercator",
    version,
    about = "Map zkSync topology from a Bridgehub root contract",
    long_about = None
)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Command,
}

#[derive(Debug, Subcommand)]
pub enum Command {
    /// Scan Bridgehub topology (CTMs + chain IDs).
    Scan(ScanArgs),
    /// Inspect a single chain deeply using bridgehub + chain ID.
    Inspect(InspectArgs),
}

#[derive(Debug, Clone, Args)]
pub struct CommonArgs {
    /// Ethereum JSON-RPC URL.
    #[arg(long, env = "MERCATOR_RPC_URL", value_parser = parse_rpc_url)]
    pub rpc_url: String,
    /// Bridgehub contract address.
    #[arg(long, value_parser = parse_address)]
    pub bridgehub: String,
    /// HTTP timeout for RPC calls.
    #[arg(long, default_value_t = 15)]
    pub timeout_secs: u64,
    /// Print additional diagnostics.
    #[arg(long, default_value_t = false)]
    pub verbose: bool,
}

#[derive(Debug, Clone, Args)]
pub struct ScanArgs {
    #[command(flatten)]
    pub common: CommonArgs,
}

#[derive(Debug, Clone, Args)]
pub struct InspectArgs {
    #[command(flatten)]
    pub common: CommonArgs,
    /// Chain ID to inspect.
    #[arg(long)]
    pub chain_id: u64,
}

pub fn parse_address(value: &str) -> Result<String, String> {
    let address = Address::from_str(value)
        .map_err(|_| "address must be 0x-prefixed and 20 bytes long".to_string())?;
    Ok(format!("{address:#x}"))
}

pub fn parse_rpc_url(value: &str) -> Result<String, String> {
    reqwest::Url::parse(value)
        .map(|url| url.to_string())
        .map_err(|err| format!("invalid rpc url: {err}"))
}

#[cfg(test)]
mod tests {
    use super::*;
    use clap::Parser;

    #[test]
    fn cli_requires_scan_flags() {
        let result = Cli::try_parse_from(["mercator", "scan"]);
        assert!(result.is_err());
    }

    #[test]
    fn cli_requires_inspect_chain_id() {
        let result = Cli::try_parse_from([
            "mercator",
            "inspect",
            "--rpc-url",
            "https://example.com",
            "--bridgehub",
            "0x0000000000000000000000000000000000000001",
        ]);
        assert!(result.is_err());
    }

    #[test]
    fn cli_parses_scan_flags() {
        let cli = Cli::try_parse_from([
            "mercator",
            "scan",
            "--rpc-url",
            "https://example.com",
            "--bridgehub",
            "0x0000000000000000000000000000000000000001",
        ])
        .expect("scan command should parse");

        let Command::Scan(args) = cli.command else {
            panic!("expected scan command");
        };
        assert_eq!(args.common.rpc_url, "https://example.com/");
        assert_eq!(
            args.common.bridgehub,
            "0x0000000000000000000000000000000000000001"
        );
        assert_eq!(args.common.timeout_secs, 15);
        assert!(!args.common.verbose);
    }

    #[test]
    fn cli_parses_inspect_flags() {
        let cli = Cli::try_parse_from([
            "mercator",
            "inspect",
            "--rpc-url",
            "https://example.com",
            "--bridgehub",
            "0x0000000000000000000000000000000000000001",
            "--chain-id",
            "324",
        ])
        .expect("inspect command should parse");

        let Command::Inspect(args) = cli.command else {
            panic!("expected inspect command");
        };
        assert_eq!(args.common.rpc_url, "https://example.com/");
        assert_eq!(
            args.common.bridgehub,
            "0x0000000000000000000000000000000000000001"
        );
        assert_eq!(args.chain_id, 324);
        assert_eq!(args.common.timeout_secs, 15);
        assert!(!args.common.verbose);
    }

    #[test]
    fn address_parser_rejects_invalid_input() {
        let result = parse_address("not_an_address");
        assert!(result.is_err());
    }
}
