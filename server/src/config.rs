use anyhow::Result;
use serde::Deserialize;
use std::fs::read_to_string;

#[derive(Clone, Deserialize)]
pub struct Config {
    pub server: ServerConfig,
    pub pg: deadpool_postgres::Config,
}

impl Config {
    /// Creates a new `Config` instance using the parameters found in the given
    /// TOML configuration file. If the file could not be found or the file is
    /// invalid, an `Error` will be returned.
    pub fn from_file(filename: &str) -> Result<Self> {
        Ok(toml::from_str(&read_to_string(filename)?)?)
    }
}

#[derive(Clone, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct ServerConfig {
    pub host: String,
    pub port: u16,
}