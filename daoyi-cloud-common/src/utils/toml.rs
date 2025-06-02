use anyhow::Context;
use daoyi_cloud_models::models::error::{AppError, Result};
use serde::de::DeserializeOwned;
use std::fs;
use std::str::FromStr;
use toml::Table;

/// Configuration management based on Toml
#[derive(Default)]
pub struct TomlConfigRegistry {
    config: Table,
}

/// The Configurable trait marks whether the struct can read configuration from the [ConfigRegistry]
pub trait Configurable {
    /// Prefix used to read toml configuration.
    /// If you need to load external configuration, you need to rewrite this method
    fn config_prefix() -> &'static str;
}

/// ConfigRegistry is the core trait of configuration management
pub trait ConfigRegistry {
    /// Get the configuration items according to the Configurable's `config_prefix`
    fn get_config<T>(&self) -> Result<T>
    where
        T: DeserializeOwned + Configurable;
}

impl ConfigRegistry for TomlConfigRegistry {
    fn get_config<T>(&self) -> Result<T>
    where
        T: DeserializeOwned + Configurable,
    {
        let prefix = T::config_prefix();
        let table = self.get_by_prefix(prefix);
        T::deserialize(table.to_owned()).map_err(|e| AppError::DeserializeErr(prefix, e))
    }
}

impl TomlConfigRegistry {
    /// Read configuration from a configuration file.
    /// If there is a configuration file corresponding to the [active environment][Env] in the same directory,
    /// the environment configuration file will be merged with the main configuration file.
    pub fn new(config_path: &str) -> Result<Self> {
        let config = Self::load_config(config_path)?;
        Ok(Self { config })
    }

    /// Get all configurations for a specified prefix
    pub fn get_by_prefix(&self, prefix: &str) -> Table {
        match self.config.get(prefix) {
            Some(toml::Value::Table(table)) => table.clone(),
            _ => Table::new(),
        }
    }

    /// load toml config
    fn load_config(config_path: &str) -> Result<Table> {
        let config_file_content = fs::read_to_string(config_path);
        let main_toml_str = match config_file_content {
            Err(e) => {
                eprintln!("Failed to read configuration file {:?}: {}", config_path, e);
                return Ok(Table::new());
            }
            Ok(content) => super::env::interpolate(&content),
        };

        let main_table = toml::from_str::<Table>(main_toml_str.as_str())
            .with_context(|| format!("Failed to parse the toml file at path {:?}", config_path))?;
        Ok(main_table)
    }
}

impl FromStr for TomlConfigRegistry {
    type Err = AppError;

    fn from_str(str: &str) -> std::result::Result<Self, Self::Err> {
        let config = toml::from_str::<Table>(str)?;
        Ok(Self { config })
    }
}
