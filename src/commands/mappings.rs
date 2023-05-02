use anyhow::Result;
use clap::{Parser, Subcommand};

use crate::application::Application;

use super::mappings_get;

#[derive(Debug, Parser)]
pub struct Arguments {
    #[command(subcommand)]
    sub_commands: Commands,
}

#[derive(Debug, Subcommand)]
enum Commands {
    /// Retrieves mapping definitions for one index
    Get(mappings_get::Arguments),
}

pub async fn handle_command(args: &Arguments, application: &Application) -> Result<()> {
    match &args.sub_commands {
        Commands::Get(args) => mappings_get::handle_command(args, application).await,
    }
}
