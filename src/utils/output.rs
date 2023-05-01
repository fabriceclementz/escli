use anyhow::{Context, Result};
use colored::Colorize;
use serde::Serialize;

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

pub fn print_success(msg: String) {
    println!("{}", msg.green())
}

pub fn print_error(msg: String) {
    println!("{}", msg.red())
}
