use std::path::PathBuf;

use serde::Deserialize;

#[derive(Deserialize)]
pub struct Config {
    pub server: ServerConfig,
    pub simulation: SimulationConfig,
}

#[derive(Deserialize)]
pub struct ServerConfig {
    pub host: String,
    pub port: u16,
    pub threads: usize,

    pub database_path: PathBuf,
}

#[derive(Deserialize)]
pub struct SimulationConfig {
    pub max_ticks: u32,
}
