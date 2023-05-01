use anyhow::Result;
use application::{Application, ApplicationArguments};
use clap::Parser;
use std::process;

mod application;
mod commands;
mod config;
mod utils;

#[macro_use]
mod macros;

#[tokio::main]
async fn main() -> Result<()> {
    pretty_env_logger::init();

    let args = ApplicationArguments::parse();
    let application = Application::new(args)?;

    match application.run().await {
        Ok(_) => {}
        Err(err) => {
            eprintln!("{:?}", err);
            process::exit(1);
        }
    }

    Ok(())
}
