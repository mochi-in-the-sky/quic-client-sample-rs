use anyhow::{anyhow, Result};
use serde::Deserialize;
use std::path::PathBuf;
use tracing::*;

const ENV_PREFIX: &str = "THROWSTERHOUSE_FIVE_";
const DEFAULT_BIND_ADDR: &str = "0.0.0.0:7777";

#[derive(Deserialize, Debug, Clone)]
pub struct Config {
    #[serde(default = "default_bind_addr")]
    pub bind_addr: String,
    pub cert_path: Option<PathBuf>,
    pub key_path: Option<PathBuf>,
}

fn default_bind_addr() -> String {
    DEFAULT_BIND_ADDR.to_string()
}

impl Config {
    pub fn new() -> Result<Self> {
        debug!("parse magic spells");
        let config = envy::prefixed(ENV_PREFIX).from_env::<Config>()?;
        if let Some(cert_path) = &config.cert_path {
            if !cert_path.exists() {
                return Err(anyhow!("cert_path not found"));
            }
        }
        if let Some(key_path) = &config.key_path {
            if !key_path.exists() {
                return Err(anyhow!("key_path not found"));
            }
        }

        Ok(config)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_env() {
        std::env::set_var("THROWSTERHOUSE_FIVE_BIND_ADDR", "0.0.0.0:1234");
        let config = Config::new().unwrap();
        assert_eq!(config.bind_addr, "0.0.0.0:1234");

        std::env::remove_var("THROWSTERHOUSE_FIVE_BIND_ADDR");
        let config = Config::new().unwrap();
        assert_eq!(config.bind_addr, "0.0.0.0:7777");

        std::env::set_var("THROWSTERHOUSE_FIVE_CERT_PATH", "/tmp/oreore.cert");
        let config = Config::new();
        assert!(config.is_err());
        std::env::remove_var("THROWSTERHOUSE_FIVE_CERT_PATH");

        std::env::set_var("THROWSTERHOUSE_FIVE_KEY_PATH", "/tmp/oreore.key");
        let config = Config::new();
        assert!(config.is_err());
        std::env::remove_var("THROWSTERHOUSE_FIVE_KEY_PATH");

        std::env::set_var("THROWSTERHOUSE_FIVE_CERT_PATH", "tests/cert/oreore.cert");
        let config = Config::new();
        assert!(config.is_ok());
        std::env::remove_var("THROWSTERHOUSE_FIVE_CERT_PATH");

        std::env::set_var("THROWSTERHOUSE_FIVE_KEY_PATH", "tests/cert/oreore.key");
        let config = Config::new();
        assert!(config.is_ok());
        std::env::remove_var("THROWSTERHOUSE_FIVE_KEY_PATH");

        std::env::remove_var("THROWSTERHOUSE_FIVE_BIND_ADDR");
        std::env::remove_var("THROWSTERHOUSE_FIVE_CERT_PATH");
        std::env::remove_var("THROWSTERHOUSE_FIVE_KEY_PATH");
    }
}
