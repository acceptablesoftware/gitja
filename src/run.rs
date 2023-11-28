use std::path::PathBuf;

use serde::Deserialize;
use serde_dhall;

#[derive(Deserialize)]
struct Config {
    repos: Vec<PathBuf>,
    scan: bool,
    template: PathBuf,
    output: PathBuf,
    host: String,
}

pub fn run(config_path: std::path::PathBuf, quiet: bool, force: bool) {
    // TODO: Catch and handle the unwrap error
    let config: Config = serde_dhall::from_file(config_path).parse().unwrap();

    println!("run");
    println!("repo_paths: {:#?}", config.repos);
    println!("scan: {:#?}", config.scan);
    println!("template: {:#?}", config.template);
    println!("output: {:#?}", config.output);
    println!("host: {:#?}", config.host);
    println!("quiet: {:#?}", quiet);
    println!("force: {:#?}", force);
}
