use anyhow::{Context, Result};
use clap::Parser;
use colored::Colorize;
use elasticsearch::cat::CatIndicesParts;
use elasticsearch::indices::IndicesGetSettingsParts;
use elasticsearch::Elasticsearch;
use futures::{stream, StreamExt};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use tabled::settings::object::Rows;
use tabled::settings::{Format, Modify, Panel, Style};
use tabled::{Table, Tabled};

use crate::application::Application;
use crate::utils::output::{output_json, Output};

#[derive(Debug, Deserialize, Serialize, Tabled)]
pub struct Index {
    #[serde(rename = "index")]
    #[tabled(display_with("Self::display_name_colorized", self))]
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
    #[tabled(display_with = "display_option")]
    version: Option<String>,
}

impl Index {
    fn display_name_colorized(&self) -> String {
        let name = &self.name;
        match self.health.as_str() {
            "yellow" => name.yellow().bold(),
            "green" => name.green().bold(),
            "red" => name.red().bold(),
            _ => name.bold(),
        }
        .to_string()
    }
}

fn display_option(o: &Option<String>) -> String {
    match o {
        Some(s) => s.to_owned(),
        None => "".into(),
    }
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

    let indices: Vec<Index> = stream::iter(indices)
        .map(|index| add_version_to_index(index, client.clone()))
        .buffer_unordered(50)
        .collect()
        .await;

    match args.output {
        Output::Default => {
            let header_format = Format::content(|s| s.bold().to_string());

            let mut table = Table::new(indices);
            table
                .with(Style::modern())
                .with(Panel::header("Indices".bold().to_string()))
                .with(Modify::new(Rows::single(1)).with(header_format));

            println!("{table}");
        }
        Output::Json => output_json(&indices, args.pretty)?,
    };

    Ok(())
}

async fn add_version_to_index(mut index: Index, client: Elasticsearch) -> Index {
    let version = get_index_version(client, &index).await.ok();
    index.version = version;
    index
}

async fn get_index_version(client: Elasticsearch, index: &Index) -> Result<String> {
    let indices_api = client.indices();

    let response = indices_api
        .get_settings(IndicesGetSettingsParts::IndexName(
            &[&index.name],
            &["index.version.created"],
        ))
        .flat_settings(true)
        .send()
        .await
        .context(format!(
            "Request error for getting index settings for {}",
            &index.name
        ))?;

    let response_body: Value = response.json().await?;

    let fallback_value = Value::String("-".to_string());
    let index_version = response_body
        .get(&index.name)
        .unwrap()
        .get("settings")
        .unwrap()
        .get("index.version.created")
        .unwrap_or(&fallback_value);

    Ok(index_version.to_string())
}
