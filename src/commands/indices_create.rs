use anyhow::{Context, Result};
use clap::{Parser, ValueEnum};
use colored::Colorize;
use elasticsearch::indices::IndicesCreateParts;
use serde_json::Value;
use tabled::builder::Builder;
use tabled::settings::Style;

use crate::application::Application;
use crate::utils::output::{output_json, print_error, print_success};

#[derive(Debug, Parser)]
pub struct CreateArgs {
    /// Name of the index to create
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

pub async fn handle_command(args: &CreateArgs, application: &Application) -> Result<()> {
    let client = application.get_http_client()?;
    let indices = client.indices();
    let create = indices.create(IndicesCreateParts::Index(&args.name));

    let response = create
        .send()
        .await
        .context(format!("Request error for creating index {}", &args.name))?;

    if !response.status_code().is_success() {
        let ex = response.exception().await?.unwrap();
        let reason = ex.error().reason().unwrap_or("");
        let status_code = ex.status().unwrap_or(0).to_string();

        match args.output {
            Output::Default => {
                print_error(format!("Index {} cannot be created!", args.name.bold()));

                let mut builder = Builder::default();
                builder
                    .set_header(["Reason", "Status Code"])
                    .push_record([reason, &status_code]);

                let mut table = builder.build();
                table.with(Style::modern());
                println!("{table}");
            }
            Output::Json => output_json(ex.error(), args.pretty)?,
        }
    } else {
        match args.output {
            Output::Default => {
                print_success(format!("Index {} created successfully!", args.name.bold()))
            }
            Output::Json => {
                let response_body: Value = response.json().await?;
                output_json(&response_body, args.pretty)?
            }
        }
    }

    Ok(())
}
