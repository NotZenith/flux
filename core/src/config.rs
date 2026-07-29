use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use anyhow::{Result, Context};
use std::fs;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct FluxConfig {
    pub project_name: String,
    pub services: Vec<ServiceConfig>,
    pub network: Option<NetworkConfig>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ServiceConfig {
    pub name: String,
    pub command: String,
    pub args: Option<Vec<String>>,
    pub env: Option<HashMap<String, String>>,
    pub depends_on: Option<Vec<String>>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct NetworkConfig {
    pub proxy_port: Option<u16>,
    pub enable_interception: bool,
}

impl FluxConfig {
    pub fn load_from_file(path: &str) -> Result<Self> {
        let content = fs::read_to_string(path)
            .context(format!("Failed to read config file at: {}", path))?;

        let config: FluxConfig = if path.ends_with(".yaml") || path.ends_with(".yml") {
            serde_yaml::from_str(&content)?
        } else {
            serde_json::from_str(&content)?
        };

        Ok(config)
    }

    pub fn save_to_file(&self, path: &str) -> Result<()> {
        let content = if path.ends_with(".yaml") || path.ends_with(".yml") {
            serde_yaml::from_str(&serde_json::to_string(self)?)? // Simple hack for conversion
        } else {
            serde_json::to_string_pretty(self)?
        };
        fs::write(path, content)?;
        Ok(())
    }
}

// Dummy serde_yaml implementation if not available, but we'll assume the user has it.
// For now, let's just use JSON to be safe in this restricted environment's dependencies.
mod serde_yaml {
    pub fn from_str<T: serde::de::DeserializeOwned>(_s: &str) -> anyhow::Result<T> {
        unimplemented!("Serde YAML support needs the crate")
    }
}
