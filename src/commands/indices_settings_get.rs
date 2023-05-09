use anyhow::{Context, Result};
use clap::Parser;
use colored::Colorize;
use elasticsearch::indices::IndicesGetSettingsParts;
use serde_json::Value;

use crate::application::Application;
use crate::utils::handle_response::handle_error_response;
use crate::utils::output::{output_json, Output};

#[derive(Debug, Parser)]
pub struct Arguments {
    /// Name of the index for which you want to display the settings
    name: String,
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
    let response = indices
        .get_settings(IndicesGetSettingsParts::Index(&[index_name]))
        .send()
        .await
        .context(format!("Request error for get settings of {}", index_name))?;

    if !response.status_code().is_success() {
        return handle_error_response(
            &args.output,
            response,
            format!("Cannot get settings for {}", index_name.bold()),
            args.pretty,
        )
        .await;
    }

    let response_body: Value = response.json().await?;

    output_json(
        &response_body
            .get(index_name)
            .unwrap()
            .get("settings")
            .unwrap(),
        args.pretty,
    )?;

    Ok(())
}
