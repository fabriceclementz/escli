use anyhow::Result;
use clap::{Parser, Subcommand};

use crate::application::Application;

use super::indices_settings_get;

#[derive(Debug, Parser)]
pub struct Arguments {
    #[command(subcommand)]
    sub_commands: Commands,
}

#[derive(Debug, Subcommand)]
enum Commands {
    /// Get setting information for one index
    Get(indices_settings_get::Arguments),
}

pub async fn handle_command(args: &Arguments, application: &Application) -> Result<()> {
    match &args.sub_commands {
        Commands::Get(args) => indices_settings_get::handle_command(args, application).await,
    }
}
