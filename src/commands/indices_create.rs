use std::fs::File;
use std::path::Path;

use anyhow::{Context, Result};
use clap::Parser;
use colored::Colorize;
use elasticsearch::indices::IndicesCreateParts;
use serde_json::json;

use crate::application::Application;
use crate::utils::handle_response::handle_response;
use crate::utils::output::Output;

#[derive(Debug, Parser)]
pub struct Arguments {
    /// Name of the index to create
    name: String,
    /// Path to a settings definition file in JSON format
    #[arg(short, long)]
    settings: Option<String>,
    /// Path to a mapping definition in JSON format
    #[arg(short, long)]
    mapping: Option<String>,
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

    let mut body = json!({});
    if let Some(mapping_path) = &args.mapping {
        let file = File::open(Path::new(&mapping_path)).context(format!(
            "Cannot open mapping definition file at {}",
            mapping_path
        ))?;

        let mapping: serde_json::Value =
            serde_json::from_reader(file).context("Malformated mapping definition")?;

        body["mappings"] = mapping;
    }

    if let Some(settings_path) = &args.settings {
        let file = File::open(Path::new(&settings_path)).context(format!(
            "Cannot open settings definition file at {}",
            settings_path
        ))?;

        let settings: serde_json::Value =
            serde_json::from_reader(file).context("Malformated settings definition")?;

        body["settings"] = json!({ "index": settings });
    }

    let response = create
        .body(body)
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
