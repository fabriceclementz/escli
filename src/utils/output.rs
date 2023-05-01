use anyhow::{Context, Result};
use serde::Serialize;

#[derive(Debug)]
pub enum JsonFormat {
    Default,
    Pretty,
}

pub fn output_json<T>(input: &T, format: JsonFormat) -> Result<()>
where
    T: Serialize,
{
    let json = match format {
        JsonFormat::Default => serde_json::to_string(&input).context("Cannot serialize as JSON")?,
        JsonFormat::Pretty => {
            serde_json::to_string_pretty(&input).context("Cannot serialize as pretty JSON")?
        }
    };

    println!("{json}");
    Ok(())
}
