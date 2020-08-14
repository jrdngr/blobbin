use serde::{Deserialize, Serialize};
use std::{fs::File, io::BufReader, path::Path};

pub const DEFAULT_CONFIG_FILE_NAME: &str = "config.json";

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub blob_size: f64,
    pub repel_force: f64,
    pub repel_distance: f64,
    pub friction_force: f64,
    pub max_acceleration: f64,
    pub min_acceleration: f64,
}

macro_rules! print_config_diff {
    ($item:expr, $new:expr) => {
        if $item != $new {
            println!("{} => {}", stringify!($item), $new);
        }
    };
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

    pub fn print_config_diff(&self, new: &Config) {
        print_config_diff!(self.blob_size, new.blob_size);
        print_config_diff!(self.repel_force, new.repel_force);
        print_config_diff!(self.repel_distance, new.repel_distance);
        print_config_diff!(self.friction_force, new.friction_force);
        print_config_diff!(self.max_acceleration, new.max_acceleration);
        print_config_diff!(self.min_acceleration, new.min_acceleration);
    }
}
