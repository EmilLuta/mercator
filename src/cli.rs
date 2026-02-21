use clap::{Args, Parser, Subcommand};

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
    /// Scan a Bridgehub and print CTMs and chain relationships.
    Scan(ScanArgs),
}

#[derive(Debug, Clone, Args)]
pub struct ScanArgs {
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

pub fn parse_address(value: &str) -> Result<String, String> {
    if value.len() != 42 || !value.starts_with("0x") {
        return Err("address must be 0x-prefixed and 20 bytes long".to_string());
    }

    if !value[2..].chars().all(|c| c.is_ascii_hexdigit()) {
        return Err("address must contain only hex characters".to_string());
    }

    Ok(value.to_ascii_lowercase())
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

        let Command::Scan(args) = cli.command;
        assert_eq!(args.rpc_url, "https://example.com/");
        assert_eq!(args.bridgehub, "0x0000000000000000000000000000000000000001");
        assert_eq!(args.timeout_secs, 15);
        assert!(!args.verbose);
    }

    #[test]
    fn address_parser_rejects_invalid_input() {
        let result = parse_address("not_an_address");
        assert!(result.is_err());
    }
}
