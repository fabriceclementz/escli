use anyhow::Result;
use clap::Parser;

use crate::application::Application;

#[derive(Debug, Parser)]
pub struct CreateArgs {
    /// name of the index to create
    name: String,
}

pub async fn handle_command(args: &CreateArgs, application: &Application) -> Result<()> {
    Ok(())
}
