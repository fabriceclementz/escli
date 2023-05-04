use anyhow::{Context, Result};
use clap::Parser;
use colored::Colorize;
use serde_json::json;

use crate::application::Application;
use crate::utils::handle_response::handle_response;
use crate::utils::output::Output;

/// Copies documents from a source to a destination
#[derive(Debug, Parser)]
pub struct Arguments {
    /// Name of the source index
    source_index: String,
    /// Name of the destination index
    dest_index: String,
    /// Output format
    #[arg(short, long, value_enum, default_value_t = Output::Default)]
    output: Output,
    /// Pretty print JSON output
    #[arg(short, long, default_value_t = false)]
    pretty: bool,
}

pub async fn handle_command(args: &Arguments, application: &Application) -> Result<()> {
    let client = application.get_http_client()?;

    // TODO: Run reindex asynchronously and poll reindex status
    let response = client
        .reindex()
        .body(json!({
            "source": {
                "index": args.source_index
            },
            "dest": {
                "index": args.dest_index
            }
        }))
        .send()
        .await
        .context(format!(
            "Request error for reindex from {} to {}",
            args.source_index, args.dest_index
        ))?;

    handle_response(
        &args.output,
        response,
        format!(
            "{} reindexed successfully to {}!",
            args.source_index.bold(),
            args.dest_index.bold()
        ),
        format!(
            "{} cannot be reindexed to {}!",
            args.source_index.bold(),
            args.dest_index.bold()
        ),
        args.pretty,
    )
    .await?;

    Ok(())
}
