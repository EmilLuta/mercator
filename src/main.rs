use clap::Parser;
use mercator::{
    cli::{Cli, Command},
    render::render_snapshot,
    rpc::HttpRpcClient,
    scanner::scan_bridgehub_ctms,
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
            let client = HttpRpcClient::new(args.rpc_url, args.timeout_secs)?;
            let snapshot = scan_bridgehub_ctms(&client, &args.bridgehub)?;
            println!("{}", render_snapshot(&snapshot, args.verbose));
        }
    }

    Ok(())
}
