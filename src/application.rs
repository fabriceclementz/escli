use crate::commands::{aliases, indices, mappings, reindex};
use crate::commands_enum;
use crate::config::Cluster;
use crate::config::Config;
use crate::ui_app::UiApp;
use anyhow::{Context, Result};
use clap::{Parser, Subcommand};
use elasticsearch::{http::transport::Transport, Elasticsearch};

#[derive(Debug, Parser)]
#[clap(author, about, version, propagate_version = true)]
pub struct ApplicationArguments {
    #[command(subcommand)]
    sub_command: Option<Commands>,

    /// Cluster to connect
    cluster: String,

    /// Config file (default is $HOME/.escli.yaml)
    #[arg(short, long, global = true)]
    config: Option<String>,

    /// Make the operation more talkative
    #[arg(short, long, default_value_t = false, global = true)]
    verbose: bool,

    /// Start escli as an interactive terminal application
    #[arg(long, default_value_t = false)]
    ui: bool,
}

// Generates the commands based on the modules in the commands directory
// Specify the modules you want to include in the commands_enum! macro
commands_enum!(indices, aliases, mappings, reindex);

#[derive(Debug)]
pub struct Application {
    config: Config,
    args: ApplicationArguments,
}

impl Application {
    pub fn new(args: ApplicationArguments) -> Result<Self> {
        let config = Config::load(args.config.as_ref())?;
        Ok(Self { config, args })
    }

    pub async fn run(&self) -> Result<()> {
        if self.args.ui {
            let mut ui_app = UiApp::new();
            ui_app.run()?;
        } else {
            Commands::run(self).await?;
            // match &self.args.sub_command {
            //     Commands::Indices(args) => indices::handle_command(args, self).await?,
            // }
        }

        Ok(())
    }

    pub fn get_http_client(&self) -> Result<Elasticsearch> {
        let current_cluster = self.get_current_cluster()?;
        let transport = Transport::single_node(&current_cluster.url())
            .context("cannot build Elasticsearch client transport layer")?;
        let client = Elasticsearch::new(transport);
        Ok(client)
    }

    fn get_current_cluster(&self) -> Result<&Cluster> {
        self.config.get_cluster_by_name(&self.args.cluster)
    }
}
