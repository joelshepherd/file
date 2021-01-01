use serde::{Deserialize, Serialize};
use std::{fs, io::Result};

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    pub recipients: Vec<String>,
    pub remote: Option<String>,
}

impl Config {
    // Create a new config
    pub fn new(recipients: Vec<String>, remote: Option<String>) -> Config {
        Config { recipients, remote }
    }

    // Read a config from disk
    pub fn read() -> Result<Config> {
        let contents = fs::read_to_string(".config")?;
        let config = toml::from_str(&contents)?;

        Ok(config)
    }

    // Write a config to disk
    pub fn write(&self, name: &str) -> Result<()> {
        let path = format!("{}/.config", name);
        let contents = toml::to_string_pretty(self).unwrap();
        fs::write(path, contents)?;

        Ok(())
    }
}
