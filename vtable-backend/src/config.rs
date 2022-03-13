//! Load configuration from a toml file.
//!

use serde::Deserialize;
use std::error;
use std::fmt;
use std::fs;

/// Real config structure. Has required fields.
#[derive(Debug)]
pub struct Config {
    pub bind_ip: String,
    pub port: u16,
}

/// Top-level config structure containing all options, used for serializing and deserializing.
/// Structured like toml file.
#[derive(Deserialize)]
pub struct TomlConfig {
    server: Option<ServerConfig>,
}

#[derive(Deserialize)]
pub struct ServerConfig {
    bind_ip: Option<String>,
    port: Option<u16>,
}

#[derive(Debug)]
pub struct ConfigError {
    message: String,
}

/// Default options to use if the config isn't specified.
mod defaults {
    pub const BIND_IP: &str = "0.0.0.0";
    pub const PORT: u16 = 8080;
}

impl TomlConfig {
    pub fn empty() -> TomlConfig {
        TomlConfig { server: None }
    }

    pub fn from_string(s: &str) -> Result<TomlConfig, ConfigError> {
        toml::from_str(s).or_else(|x| Err(ConfigError::new(&x.to_string())))
    }

    pub fn from_file(file: &str) -> Result<TomlConfig, ConfigError> {
        let contents = match fs::read_to_string(file) {
            Ok(x) => x,
            Err(e) => return Err(ConfigError::new(&e.to_string())),
        };

        TomlConfig::from_string(&contents)
    }

    pub fn parse(&self) -> Config {
        // Configure defaults

        // This sequence Checks if the ServerConfig option is set. If it is, it gets the bind_ip Option. If either is None, gets the default.
        let bind_ip: String = self
            .server
            .as_ref()
            .and_then(|x| x.bind_ip.clone())
            .unwrap_or(String::from(defaults::BIND_IP));

        let port = self
            .server
            .as_ref()
            .and_then(|x| x.port)
            .unwrap_or(defaults::PORT);

        Config { bind_ip, port }
    }
}

impl ConfigError {
    pub fn new(message: &str) -> ConfigError {
        ConfigError {
            message: String::from(message),
        }
    }
}

impl fmt::Display for ConfigError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Problem reading configuration: {}", &self.message)
    }
}
impl error::Error for ConfigError {}
