use anyhow::{Context, Result};
use serde::Deserialize;
use std::collections::HashMap;
use std::fs;

#[derive(Debug, Deserialize)]

pub struct StructuredRepos {
    pub repos: Vec<String>,
}

#[derive(Debug, Deserialize)]
pub struct Config {
    pub folders: HashMap<String, StructuredRepos>,
}

pub fn load_config(path: &str) -> Result<Config> {
    let expanded_path: String = shellexpand::tilde(path).to_string();
    let config: Config = toml::from_str(
        &fs::read_to_string(&expanded_path)
            .with_context(|| format!("failed to read config file: {}", expanded_path))?,
    )
    .with_context(|| format!("failed to parse the toml from: {}", expanded_path))?;
    Ok(config)
}
