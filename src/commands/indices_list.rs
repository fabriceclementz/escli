use anyhow::{Context, Result};
use clap::Parser;
use elasticsearch::cat::CatIndicesParts;
use elasticsearch::indices::IndicesGetSettingsParts;
use elasticsearch::Elasticsearch;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use tabled::settings::{Panel, Style};
use tabled::{Table, Tabled};

use crate::application::Application;
use crate::utils::output::{output_json, Output};

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
    #[tabled(display_with = "display_option")]
    version: Option<String>,
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

    let mut indices: Vec<Index> = response
        .json()
        .await
        .context("Cannot parse JSON response for indices list")?;

    //  TODO: improve with StreamExt
    for index in &mut indices {
        let index_version = get_index_version(&client, index).await?;
        index.version = Some(index_version.to_string());
    }

    match args.output {
        Output::Default => {
            let mut table = Table::new(indices);
            table.with(Style::modern()).with(Panel::header("Indices"));
            println!("{table}");
        }
        Output::Json => output_json(&indices, args.pretty)?,
    };

    Ok(())
}

async fn get_index_version(client: &Elasticsearch, index: &Index) -> Result<String> {
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

    let index_version = response_body
        .get(&index.name)
        .unwrap()
        .get("settings")
        .unwrap()
        .get("index.version.created")
        .unwrap();

    Ok(index_version.to_string())
}
