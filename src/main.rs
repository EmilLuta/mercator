use clap::{Parser, Subcommand};

#[derive(Debug, Parser)]
#[command(
    name = "mercator",
    version,
    about = "Map on-chain systems from a known root contract",
    long_about = None
)]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Debug, Subcommand)]
enum Command {
    /// Print repository bootstrap information.
    Init,
    /// Placeholder for future contract inspection work.
    Probe {
        /// Root contract address to start from (0x-prefixed).
        address: String,
    },
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Command::Init => {
            println!("mercator is ready. Next: implement your first zkSync extractor.");
        }
        Command::Probe { address } => {
            println!("probe is not implemented yet. received root address: {address}");
        }
    }
}
