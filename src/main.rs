use clap::Parser;
use mercator::{
    cli::{Cli, Command},
    render::{render_chain_inspection, render_topology},
    rpc::HttpRpcClient,
    scanner::{inspect_bridgehub_chain, scan_bridgehub_topology},
};

fn main() {
    if let Err(err) = run() {
        eprintln!("error: {err}");
        std::process::exit(1);
    }
}

fn run() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    match cli.command {
        Command::Scan(args) => {
            let client = HttpRpcClient::new(args.common.rpc_url, args.common.timeout_secs)?;
            let snapshot = scan_bridgehub_topology(&client, &args.common.bridgehub)?;
            println!("{}", render_topology(&snapshot, args.common.verbose));
        }
        Command::Inspect(args) => {
            let client = HttpRpcClient::new(args.common.rpc_url, args.common.timeout_secs)?;
            let inspection =
                inspect_bridgehub_chain(&client, &args.common.bridgehub, args.chain_id)?;
            println!(
                "{}",
                render_chain_inspection(&inspection, args.common.verbose)
            );
        }
    }

    Ok(())
}
