use std::fs;
use std::path::Path;
use anyhow::Result;
use crate::config::Config;

pub fn load_config<P: AsRef<Path>>(path: P) -> Result<Config> {
    let config_str = fs::read_to_string(path)?;
    let config: Config = toml::from_str(&config_str)?;
    Ok(config)
}