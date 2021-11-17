use std::error::Error;
use std::fs;

use serde::{Deserialize, Serialize};
use serde_yaml;

#[derive(Serialize, Deserialize)]
pub struct Config {
    pub api_key: String
}

/// Parse configuration file
pub fn parse_config(path: &str) -> Result<Config, Box<dyn Error>> {
    let data: String = fs::read_to_string(path)?;
    let c: Config = serde_yaml::from_str(data.as_str())?;
    Ok(c)
}
