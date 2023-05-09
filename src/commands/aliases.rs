use anyhow::Result;
use clap::{Parser, Subcommand};

use crate::application::Application;

use super::aliases_add;
use super::aliases_list;
use super::aliases_remove;
use super::aliases_update;

#[derive(Debug, Parser)]
pub struct Arguments {
    #[command(subcommand)]
    sub_commands: Commands,
}

#[derive(Debug, Subcommand)]
enum Commands {
    /// List all aliases
    List(aliases_list::Arguments),
    /// Add an index to an alias
    Add(aliases_add::Arguments),
    /// Remove an index from an alias
    Remove(aliases_remove::Arguments),
    /// Updates aliases for an index
    Update(aliases_update::Arguments),
}

pub async fn handle_command(args: &Arguments, application: &Application) -> Result<()> {
    match &args.sub_commands {
        Commands::List(args) => aliases_list::handle_command(args, application).await,
        Commands::Add(args) => aliases_add::handle_command(args, application).await,
        Commands::Remove(args) => aliases_remove::handle_command(args, application).await,
        Commands::Update(args) => aliases_update::handle_command(args, application).await,
    }
}
