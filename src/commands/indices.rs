use anyhow::Result;
use clap::{Parser, Subcommand};

use crate::application::Application;

use super::indices_create;
use super::indices_list;

#[derive(Debug, Parser)]
pub struct Arguments {
    #[command(subcommand)]
    sub_commands: Commands,
}

// commands_enum!(
//     list,
//     create,
// );

#[derive(Debug, Subcommand)]
enum Commands {
    /// List all indices
    List(indices_list::ListArgs),
    /// Create an index
    Create(indices_create::CreateArgs),
}

pub async fn handle_command(args: &Arguments, application: &Application) -> Result<()> {
    match &args.sub_commands {
        Commands::List(args) => indices_list::handle_command(args, application).await,
        Commands::Create(args) => indices_create::handle_command(args, application).await,
    }
}
