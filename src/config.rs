use std::{
    path::Path,
    io::BufReader,
    fs::File,
};
use serde::{Serialize, Deserialize};

pub const DEFAULT_CONFIG_FILE_NAME: &str = "config.json";

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub blob_size: f64,
    pub repel_force: f64,
    pub repel_distance: f64,
    pub friction_force: f64,
    pub max_acceleration: f64,
}

impl Config {
    pub fn load_file<P: AsRef<Path>>(path: P) -> anyhow::Result<Self> {
        let file = File::open(path)?;
        let reader = BufReader::new(file);

        let config = serde_json::from_reader(reader)?;

        Ok(config)
    }

    pub fn load_default_config_file() -> anyhow::Result<Self> {
        Config::load_file(DEFAULT_CONFIG_FILE_NAME)
    }
}
