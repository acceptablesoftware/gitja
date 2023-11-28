use std::include_bytes;
use std::path::Path;

use include_dir::{include_dir, Dir};

static BASE_TEMPLATE: Dir<'_> = include_dir!("$CARGO_MANIFEST_DIR/templates/base");
static CONFIG: &'static [u8] = include_bytes!("../config.dhall");

const MESSAGE: &str = "A base template as been put at ./template.
A plain config has been put at ./config.dhall.
Add a local git repository to 'repos' in the config and run gitja to generate HTML in ./output.";

pub fn init() {
    let template_path = Path::new("./template");

    if template_path.exists() {
        println!("Failed - ./template already exists.");
        return;
    }

    let config_path = Path::new("./config.dhall");

    if config_path.exists() {
        println!("Failed - ./config.dhall already exists.");
        return;
    }

    std::fs::create_dir(template_path).unwrap();
    BASE_TEMPLATE.extract(template_path).unwrap();
    std::fs::write(config_path, CONFIG).unwrap();

    println!("{}", MESSAGE);

    if Path::new("./output").exists() {
        println!("WARNING: ./output ALREADY EXISTS AND WILL BE OVERWRITTEN");
        println!("UNLESS YOU MOVE/RENAME IT OR CHANGE gitja'S OUTPUT.");
    }
}
