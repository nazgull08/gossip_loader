use serde::Deserialize;
use std::fs;
use std::path::Path;

#[derive(Debug, Clone, Deserialize)]
pub struct LoaderConfig {
    pub server: ServerConfig,
    pub load: LoadConfig,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ServerConfig {
    pub connect_addr: String,
    pub json_path: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct LoadConfig {
    pub clients: usize,
    pub interval_ms: u64,
    pub duration_secs: u64,
    pub pattern: LoadPattern,
}


#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum LoadPattern {
    Steady,
    Burst,
    RampUp,
}


impl LoaderConfig {
    pub fn from_file<P: AsRef<Path>>(path: P) -> Self {
        let contents = fs::read_to_string(path).expect("Failed to read config file");
        toml::from_str(&contents).expect("Invalid TOML format")
    }
}

