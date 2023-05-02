use anyhow::{Context, Result};
use clap::ValueEnum;
use colored::Colorize;
use serde::Serialize;
use tabled::{builder::Builder, settings::Style};

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
pub enum Output {
    /// Display in human friendly format
    Default,
    /// Displays output as JSON
    Json,
}

pub fn output_json<T>(input: &T, pretty: bool) -> Result<()>
where
    T: Serialize,
{
    let json = match pretty {
        false => serde_json::to_string(&input).context("Cannot serialize as JSON")?,
        true => serde_json::to_string_pretty(&input).context("Cannot serialize as pretty JSON")?,
    };

    println!("{json}");
    Ok(())
}

pub fn output_error_table(reason: &str, status_code: &str) {
    let mut builder = Builder::default();
    builder
        .set_header(["Reason", "Status Code"])
        .push_record([reason, status_code]);

    let mut table = builder.build();
    table.with(Style::modern());
    println!("{table}");
}

pub fn print_success(msg: String) {
    println!("{}", msg.green())
}

pub fn print_error(msg: String) {
    println!("{}", msg.red())
}
