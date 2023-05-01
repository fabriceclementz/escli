use anyhow::{Context, Result};
use clap::{Parser, ValueEnum};
use colored::Colorize;
use elasticsearch::indices::IndicesOpenParts;
use serde_json::Value;

use crate::application::Application;
use crate::utils::output::{output_error_table, output_json, print_error, print_success};

#[derive(Debug, Parser)]
pub struct OpenArgs {
    /// Name of the index to open
    name: String,
    /// Output format
    #[arg(short, long, value_enum, default_value_t = Output::Default)]
    output: Output,
    /// Pretty print JSON output
    #[arg(short, long, default_value_t = false)]
    pretty: bool,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
enum Output {
    /// Display indices in table format
    Default,
    /// Displays output as JSON
    Json,
}

pub async fn handle_command(args: &OpenArgs, application: &Application) -> Result<()> {
    let client = application.get_http_client()?;
    let indices = client.indices();

    let response = indices
        .open(IndicesOpenParts::Index(&[&args.name]))
        .send()
        .await
        .context(format!("Request error for opening index {}", &args.name))?;

    if !response.status_code().is_success() {
        let ex = response.exception().await?.unwrap();
        let reason = ex.error().reason().unwrap_or("");
        let status_code = ex.status().unwrap_or(0).to_string();

        match args.output {
            Output::Default => {
                print_error(format!("Index {} cannot be opened!", args.name.bold()));
                output_error_table(reason, &status_code);
            }
            Output::Json => output_json(ex.error(), args.pretty)?,
        };
    } else {
        match args.output {
            Output::Default => {
                print_success(format!("Index {} opened successfully!", args.name.bold()))
            }
            Output::Json => {
                let response_body: Value = response.json().await?;
                output_json(&response_body, args.pretty)?
            }
        }
    }

    Ok(())
}
