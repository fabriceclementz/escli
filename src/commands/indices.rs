use anyhow::Result;
use clap::{Parser, Subcommand};

use crate::application::Application;

use super::indices_close;
use super::indices_create;
use super::indices_delete;
use super::indices_list;
use super::indices_open;
use super::indices_settings;

#[derive(Debug, Parser)]
pub struct Arguments {
    #[command(subcommand)]
    sub_commands: Commands,
}

#[derive(Debug, Subcommand)]
enum Commands {
    /// List all indices
    List(indices_list::Arguments),
    /// Create an index
    Create(indices_create::Arguments),
    /// Deletes an index
    Delete(indices_delete::Arguments),
    /// Opens a closed index
    Open(indices_open::Arguments),
    /// Closes an index
    Close(indices_close::Arguments),
    /// Interacts with index settings
    Settings(indices_settings::Arguments),
}

pub async fn handle_command(args: &Arguments, application: &Application) -> Result<()> {
    match &args.sub_commands {
        Commands::List(args) => indices_list::handle_command(args, application).await,
        Commands::Create(args) => indices_create::handle_command(args, application).await,
        Commands::Open(args) => indices_open::handle_command(args, application).await,
        Commands::Close(args) => indices_close::handle_command(args, application).await,
        Commands::Delete(args) => indices_delete::handle_command(args, application).await,
        Commands::Settings(args) => indices_settings::handle_command(args, application).await,
    }
}
