use once_cell::sync::Lazy;
use std::collections::HashMap;
use std::fmt;
use std::fs;

use serde_yaml;

/// Here ere stored the test-environment related parameters.
/// Those are saved in a yaml file.
/// The name of this file is set a environment parameter `ENV_EXECUTION`.
/// We can have several such configuration yaml files likes,
/// one fore ach environment (dev, test, pre-prod, prod)
pub static CONFIG_MAP: Lazy<ConfigMap> = Lazy::new(initialize_config_map);

pub type ConfigMap = HashMap<String, String>;

// Function to initialize the HashMap from YAML file
pub fn initialize_config_map() -> ConfigMap {
    let config_file_path =
        std::env::var("ENV_EXECUTION").expect("ENV_EXECUTION variable needs to be set");

    // Read YAML file into a string
    let contents = fs::read_to_string(config_file_path).expect("Failed to read config file");
    // Deserialize YAML directly into ConfigMap
    serde_yaml::from_str(&contents).expect("Failed to parse YAML")
}

// Custom error type
#[derive(Debug)]
pub enum ConfigError {
    MissingKey(String),
}

impl fmt::Display for ConfigError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ConfigError::MissingKey(ref key) => write!(f, "key not found: {}", key),
        }
    }
}

impl std::error::Error for ConfigError {}

// Function to retrieve a value from CONFIG_MAP based on key
pub fn get_config_value(key: &str) -> Result<String, ConfigError> {
    CONFIG_MAP
        .get(key)
        .map(|v| v.to_string())
        .ok_or(ConfigError::MissingKey(key.to_string()))
}
