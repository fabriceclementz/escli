use anyhow::{bail, Context, Result};
use log::debug;
use serde::Deserialize;
use std::{
    collections::HashMap,
    fmt::Display,
    fs::File,
    path::{Path, PathBuf},
};

#[derive(Debug, Deserialize)]
pub struct Config {
    clusters: HashMap<String, Cluster>,
}

impl Config {
    /// Loads config file from path or $HOME/.config/escli.yaml
    pub fn load(path: Option<&String>) -> Result<Self> {
        if let Some(path) = path {
            debug!("Loading application config from {}", path);
            let config = read_config_file(&Path::new(path).to_path_buf())?;
            return Ok(config);
        }

        debug!("Loading application config from ~/.escli/config.yaml");
        let home_dir = dirs::home_dir().context("Unable to get home directory")?;
        let config_path = home_dir.join(Path::new(".escli/config.yaml"));

        let config = read_config_file(&config_path)?;
        Ok(config)
    }

    pub fn get_cluster_by_name(&self, name: &str) -> Result<&Cluster> {
        match self.clusters.get(name) {
            Some(cluster) => Ok(cluster),
            None => bail!(
                "Invalid cluster name '{}'. Valid names are {}.",
                name,
                self.get_valid_cluster_names().join(",")
            ),
        }
    }

    fn get_valid_cluster_names(&self) -> Vec<String> {
        self.clusters.keys().cloned().collect()
    }
}

#[derive(Debug, Deserialize)]
pub struct Cluster {
    host: String,
    port: Option<usize>,
    protocol: Option<Protocol>,
}

impl Cluster {
    pub fn url(&self) -> String {
        let protocol = &self.protocol.unwrap_or(Protocol::Https).to_string();
        let port = &self.port.unwrap_or(9200);
        let host = &self.host;

        format!("{protocol}://{host}:{port}")
    }
}

#[derive(Debug, Deserialize, Clone, Copy)]
enum Protocol {
    #[serde(rename = "http")]
    Http,
    #[serde(rename = "https")]
    Https,
}

impl Display for Protocol {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Protocol::Http => f.write_str("http"),
            Protocol::Https => f.write_str("https"),
        }
    }
}

fn read_config_file(path: &PathBuf) -> Result<Config> {
    let config_file =
        File::open(&path).context(format!("Unable to open config file at {:?}", path))?;

    let config: Config = serde_yaml::from_reader(config_file)
        .context(format!("Unable to deserialize config file at {:?}", path))?;

    Ok(config)
}
