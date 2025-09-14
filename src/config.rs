use std::env;
use std::fs;

use serde::Deserialize;

const DEFAULT_CONFIG_PATH: &str = "/etc/bluegent.conf";

#[derive(Deserialize)]
pub struct Config {
    pub pin_code: String,
    pub authorized_services: Vec<String>,
}

impl Config {
    pub fn load() -> Self {
        let config_path = env::var("BLUEGENT_CONFIG").unwrap_or(String::from(DEFAULT_CONFIG_PATH));

        let config_text = fs::read_to_string(&config_path)
            .expect(format!("failed to read config from {}", config_path).as_str());

        toml::from_str(config_text.as_str())
            .expect(format!("failed to parse config in {}", config_path).as_str())
    }
}
