use anyhow::{Context, Result};
use clap::Parser;
use colored::Colorize;
use elasticsearch::cat::CatAliasesParts;
use serde::{Deserialize, Serialize};
use tabled::settings::object::Rows;
use tabled::settings::{Format, Modify, Panel, Style};
use tabled::{Table, Tabled};

use crate::application::Application;
use crate::utils::output::{output_json, Output};

#[derive(Debug, Deserialize, Serialize, Tabled)]
pub struct Alias {
    alias: String,
    index: String,
    filter: String,
    #[serde(rename = "routing.index")]
    routing_index: String,
    #[serde(rename = "routing.search")]
    routing_search: String,
}

#[derive(Parser, Debug)]
pub struct Arguments {
    /// Output format
    #[arg(short, long, value_enum, default_value_t = Output::Default)]
    output: Output,

    /// Pretty print JSON output
    #[arg(short, long, default_value_t = false)]
    pretty: bool,
}

pub async fn handle_command(args: &Arguments, application: &Application) -> Result<()> {
    let client = application.get_http_client()?;
    let cat = client.cat();

    let response = cat
        .aliases(CatAliasesParts::None)
        .h(&[
            "alias",
            "index",
            "filter",
            "routing.index",
            "routing.search",
        ])
        .format("json")
        .send()
        .await
        .context("Request error for getting aliases list")?;

    let aliases: Vec<Alias> = response
        .json()
        .await
        .context("Cannot parse JSON response for aliases list")?;

    match args.output {
        Output::Default => {
            let header_format = Format::content(|s| s.bold().to_string());

            let mut table = Table::new(aliases);
            table
                .with(Style::modern())
                .with(Panel::header("Aliases".bold().to_string()))
                .with(Modify::new(Rows::single(1)).with(header_format));

            println!("{table}");
        }
        Output::Json => output_json(&aliases, args.pretty)?,
    };

    Ok(())
}
