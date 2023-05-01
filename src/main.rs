use anyhow::Result;
use clap::{Parser, Subcommand};

#[derive(Parser)]
#[clap(author, about, version, propagate_version = true)]
struct Arguments {
    #[command(subcommand)]
    sub_command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Interact with indices
    Indices,
    /// Interact with aliases
    Aliases,
    /// Interact with nodes
    Nodes,
}

#[tokio::main]
async fn main() -> Result<()> {
    pretty_env_logger::init();

    let args = Arguments::parse();

    match args.sub_command {
        Commands::Indices => todo!(),
        Commands::Aliases => todo!(),
        Commands::Nodes => todo!(),
    };

    Ok(())
}
