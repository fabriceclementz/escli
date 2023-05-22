use std::time::{Duration, SystemTime};

use anyhow::{bail, Context, Result};
use clap::Parser;
use colored::Colorize;
use elasticsearch::tasks::TasksGetParts;
use elasticsearch::Elasticsearch;
use indicatif::{ProgressBar, ProgressStyle};
use serde_json::{json, Value};

use crate::application::Application;
use crate::utils::handle_response::handle_error_response;
use crate::utils::output::{output_json, print_success, Output};

/// Copies documents from a source to a destination
#[derive(Debug, Parser)]
pub struct Arguments {
    /// Name of the source index
    source_index: String,
    /// Name of the destination index
    dest_index: String,
    /// Output format
    #[arg(short, long, value_enum, default_value_t = Output::Default)]
    output: Output,
    /// Pretty print JSON output
    #[arg(short, long, default_value_t = false)]
    pretty: bool,
}

pub async fn handle_command(args: &Arguments, application: &Application) -> Result<()> {
    let client = application.get_http_client()?;

    // TODO: Run reindex asynchronously and poll reindex status
    let response = client
        .reindex()
        .wait_for_completion(false)
        .body(json!({
            "source": {
                "index": args.source_index
            },
            "dest": {
                "index": args.dest_index
            }
        }))
        .send()
        .await
        .context(format!(
            "Request error for reindex from {} to {}",
            args.source_index, args.dest_index
        ))?;

    if !response.status_code().is_success() {
        handle_error_response(
            &args.output,
            response,
            format!(
                "{} cannot be reindexed to {}!",
                args.source_index.bold(),
                args.dest_index.bold()
            ),
            args.pretty,
        )
        .await?
    } else {
        let response_body: Value = response.json().await?;

        let task_id = match response_body.get("task") {
            Some(task) => task.to_string().replace("\"", ""),
            None => bail!("Missing task in reindex response"),
        };

        let start = SystemTime::now();
        let task = poll_task(&task_id, &client).await?;
        let reindex_duration = start.elapsed()?;

        match args.output {
            Output::Default => print_success(format!(
                "Reindex finished successfully! ({}s)",
                reindex_duration.as_secs()
            )),
            Output::Json => output_json(&task, args.pretty)?,
        }
    }

    Ok(())
}

async fn poll_task(task_id: &str, client: &Elasticsearch) -> Result<Value> {
    let pb = ProgressBar::new_spinner();
    pb.enable_steady_tick(Duration::from_millis(100));
    pb.set_style(
        ProgressStyle::with_template("{spinner:.blue} {msg}")
            .unwrap()
            // For more spinners check out the cli-spinners project:
            // https://github.com/sindresorhus/cli-spinners/blob/master/spinners.json
            .tick_strings(&[
                "🕛 ", "🕚 ", "🕙 ", "🕘 ", "🕗 ", "🕖 ", "🕕 ", "🕔 ", "🕓 ", "🕒 ", "🕑 ", "🕐 ",
            ]),
    );
    pb.set_message("Reindexing...");

    let mut interval = tokio::time::interval(Duration::from_secs(15));

    loop {
        interval.tick().await;

        let task = get_task_by_id(task_id, client).await?;
        // println!("{:?}", task);

        match task.get("completed") {
            Some(completed) if completed == true => {
                pb.finish_and_clear();
                return Ok(task);
            }
            None => {
                pb.finish_and_clear();
                bail!("Cannot get completion state for task id: {}", task_id)
            }
            _ => {}
        }
    }
}

async fn get_task_by_id(task_id: &str, client: &Elasticsearch) -> Result<Value> {
    let response = client
        .tasks()
        .get(TasksGetParts::TaskId(&task_id))
        .send()
        .await
        .context(format!("Cannot get task response for id {}", task_id))?;

    if !response.status_code().is_success() {
        bail!("Cannot get task for id: {}", task_id);
    }

    let response_body: Value = response.json().await?;

    Ok(response_body)
}
