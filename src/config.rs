use serde::{Deserialize, Serialize};
use std::{fs, io::Result};

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    pub recipients: Vec<String>,
    pub remote: Option<String>,
}

impl Config {
    pub fn new(recipients: Vec<String>, remote: Option<String>) -> Config {
        Config { recipients, remote }
    }
}

pub fn read() -> Result<Config> {
    let contents = fs::read_to_string(".config")?;
    let config = toml::from_str(&contents)?;

    Ok(config)
}

pub fn write(name: &str, config: &Config) -> Result<()> {
    let path = format!("{}/.config", name);
    let contents = toml::to_string_pretty(config).unwrap();
    fs::write(path, contents)?;

    Ok(())
}
