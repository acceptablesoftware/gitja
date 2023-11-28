use std::path::PathBuf;

use crate::config;

pub fn run(config_path: PathBuf, quiet: bool, force: bool) {
    let conf = config::load_config(config_path);

    println!("run");
    println!("repo_paths: {:#?}", conf.repos);
    println!("scan: {:#?}", conf.scan);
    println!("template: {:#?}", conf.template);
    println!("output: {:#?}", conf.output);
    println!("host: {:#?}", conf.host);
    println!("quiet: {:#?}", quiet);
    println!("force: {:#?}", force);
}
