use anyhow::{Context, Result};
use clap::Parser;
use colored::Colorize;
use elasticsearch::indices::IndicesDeleteParts;

use crate::application::Application;
use crate::utils::handle_response::handle_response;
use crate::utils::output::Output;

#[derive(Debug, Parser)]
pub struct Arguments {
    /// Name of the index to delete
    name: String,
    /// Output format
    #[arg(short, long, value_enum, default_value_t = Output::Default)]
    output: Output,

    /// Pretty print JSON output
    #[arg(short, long, default_value_t = false)]
    pretty: bool,
}

pub async fn handle_command(args: &Arguments, application: &Application) -> Result<()> {
    let index_name = &args.name;
    let client = application.get_http_client()?;
    let indices = client.indices();

    let response = indices
        .delete(IndicesDeleteParts::Index(&[index_name]))
        .send()
        .await
        .context(format!("Request error for deleting index {}", index_name))?;

    handle_response(
        &args.output,
        response,
        format!("Index {} deleted successfully!", index_name.bold()),
        format!("Index {} cannot be deleted!", index_name.bold()),
        args.pretty,
    )
    .await
}
