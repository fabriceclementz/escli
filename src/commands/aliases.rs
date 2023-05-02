use anyhow::Result;
use clap::{Parser, Subcommand};

use crate::application::Application;

use super::aliases_list;

#[derive(Debug, Parser)]
pub struct Arguments {
    #[command(subcommand)]
    sub_commands: Commands,
}

#[derive(Debug, Subcommand)]
enum Commands {
    /// List all aliases
    List(aliases_list::Arguments),
}

pub async fn handle_command(args: &Arguments, application: &Application) -> Result<()> {
    match &args.sub_commands {
        Commands::List(args) => aliases_list::handle_command(args, application).await,
    }
}
