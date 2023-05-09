use crate::{
    application::Application,
    utils::{handle_response::handle_response, output::Output},
};
use anyhow::{Context, Result};
use clap::Parser;
use colored::Colorize;
use serde_json::json;

#[derive(Debug, Parser)]
pub struct Arguments {
    /// Alias
    alias: String,
    /// Index you want to remove from alias
    index: String,
    /// Output format
    #[arg(short, long, value_enum, default_value_t = Output::Default)]
    output: Output,
    /// Pretty print JSON output
    #[arg(short, long, default_value_t = false)]
    pretty: bool,
}

pub async fn handle_command(args: &Arguments, application: &Application) -> Result<()> {
    let client = application.get_http_client()?;
    let response = client
        .indices()
        .update_aliases()
        .body(json!({
            "actions": [
                {
                    "remove": {
                        "index": args.index,
                        "alias": args.alias
                    }
                }
            ]
        }))
        .send()
        .await
        .context("Request error for removing alias")?;

    handle_response(
        &args.output,
        response,
        format!(
            "Index {} removed from alias {} successfully!",
            args.index.bold(),
            args.alias.bold()
        ),
        format!(
            "Index {} cannot be removed from alias {}",
            args.index.bold(),
            args.alias.bold()
        ),
        args.pretty,
    )
    .await?;

    Ok(())
}
