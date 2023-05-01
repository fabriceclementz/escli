use anyhow::{Context, Result};
use clap::{Parser, ValueEnum};
use elasticsearch::cat::CatIndicesParts;
use serde::{Deserialize, Serialize};
use tabled::{
    settings::{Panel, Style},
    Table, Tabled,
};

use crate::application::Application;
use crate::utils::output::{output_json, JsonFormat};

#[derive(Debug, Deserialize, Serialize, Tabled)]
pub struct Index {
    #[serde(rename = "index")]
    name: String,
    health: String,
    status: String,
    pri: String,
    rep: String,
    #[serde(rename = "store.size")]
    #[tabled(display_with = "display_option")]
    size: Option<String>,
    #[serde(rename = "docs.count")]
    #[tabled(display_with = "display_option")]
    docs_count: Option<String>,
}

fn display_option(o: &Option<String>) -> String {
    match o {
        Some(s) => s.to_owned(),
        None => "".into(),
    }
}

#[derive(Parser, Debug)]
pub struct ListArgs {
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

pub async fn handle_command(args: &ListArgs, application: &Application) -> Result<()> {
    let client = application.get_http_client()?;
    let cat = client.cat();
    let response = cat
        .indices(CatIndicesParts::None)
        .h(&[
            "index",
            "status",
            "health",
            "pri",
            "rep",
            "store.size",
            "docs.count",
        ])
        .format("json")
        .pretty(true)
        .send()
        .await
        .context("Request error for getting indices list")?;

    let indices: Vec<Index> = response
        .json()
        .await
        .context("Cannot parse JSON response for indices list")?;

    match args.output {
        Output::Default => {
            let mut table = Table::new(indices);
            table.with(Style::modern()).with(Panel::header("Indices"));
            println!("{}", table.to_string());
        }
        Output::Json => {
            match args.pretty {
                true => output_json(&indices, JsonFormat::Pretty).context("serialize indices")?,
                false => output_json(&indices, JsonFormat::Default).context("serialize indices")?,
            };
        }
    };

    Ok(())
}
