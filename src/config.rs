use anyhow::{bail, Result};
use log::debug;
use serde::Deserialize;
use std::{collections::HashMap, fmt::Display};

#[derive(Debug, Deserialize)]
pub struct Config {
    clusters: HashMap<String, Cluster>,
}

impl Config {
    pub fn load(path: Option<&String>) -> Result<Self> {
        debug!("Loading application config");

        // TODO: load config file from path or $HOME/.config/escli.yaml
        let mut clusters = HashMap::new();
        clusters.insert(
            "local".into(),
            Cluster {
                host: "127.0.0.1".into(),
                port: Some(9200),
                protocol: Some(Protocol::Http),
            },
        );

        Ok(Self { clusters })
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
