use anyhow::{Context, Result};
use clap::Parser;
use colored::Colorize;
use elasticsearch::indices::IndicesCreateParts;

use crate::application::Application;
use crate::utils::handle_response::handle_response;
use crate::utils::output::Output;

#[derive(Debug, Parser)]
pub struct Arguments {
    /// Name of the index to create
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
    let create = indices.create(IndicesCreateParts::Index(index_name));

    let response = create
        .send()
        .await
        .context(format!("Request error for creating index {}", index_name))?;

    handle_response(
        &args.output,
        response,
        format!("Index {} created successfully!", index_name.bold()),
        format!("Index {} cannot be created!", index_name.bold()),
        args.pretty,
    )
    .await?;

    Ok(())
}
