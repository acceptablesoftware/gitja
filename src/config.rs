use std::path::PathBuf;

use serde::Deserialize;
use serde_dhall;

#[derive(Deserialize)]
pub struct Config {
    pub repos: Vec<PathBuf>,
    pub scan: bool,
    pub template: PathBuf,
    pub output: PathBuf,
    pub host: String,
}

pub fn load_config(config_path: PathBuf) -> Config {
    // TODO: Catch and handle the unwrap error
    let config: Config = serde_dhall::from_file(config_path).parse().unwrap();
    config
}
