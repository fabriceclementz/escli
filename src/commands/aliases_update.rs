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
    /// Alias you want to delete from the index
    old_alias: String,
    /// Alias you want to add the index
    new_alias: String,
    /// Index you want to update aliases
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
                    "add": {
                        "index": args.index,
                        "alias": args.new_alias
                    }
                },
                {
                    "remove": {
                        "index": args.index,
                        "alias": args.old_alias
                    }
                }
            ]
        }))
        .send()
        .await
        .context("Request error for updating alias")?;

    handle_response(
        &args.output,
        response,
        format!(
            "Index aliases updated successfully! {} -> {}",
            args.index.bold(),
            args.new_alias.bold(),
        ),
        format!("Index aliases cannot be updated",),
        args.pretty,
    )
    .await?;

    Ok(())
}
