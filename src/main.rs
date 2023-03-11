use std::borrow::Cow;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::ExitCode;

use clap::Parser;
use rust_embed::RustEmbed;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// Run gitja with the specified config file.
    #[arg(short, long, default_value = "config.dhall", value_name = "FILE")]
    config: PathBuf,

    /// Suppress non-error output.
    #[arg(short, long)]
    quiet: bool,

    /// Force regeneration of all files.
    #[arg(short, long)]
    force: bool,

    /// Create a basic template with config in the current folder.
    #[arg(short, long)]
    init: bool,
}

fn main() -> ExitCode {
    let cli = Cli::parse();

    if cli.init {
        if init() {
            return ExitCode::SUCCESS;
        } else {
            return ExitCode::from(1);
        }
    }
    ExitCode::SUCCESS
}

#[derive(RustEmbed)]
#[folder = "."]
#[include = "templates/base/*"]
#[include = "config.dhall"]
struct Assets;

fn init() -> bool {
    if ["./template", "./config.dhall"]
        .iter()
        .map(Path::new)
        .filter(|p| {
            p.try_exists()
                .unwrap_or_else(|_| panic!("Failed to check existence of {}.", p.display()))
        })
        .map(|p| println!("Found path: {}", p.display()))
        .fold(false, |_, _| true)
    {
        println!("Please remove to continue.");
        false
    } else {
        if Assets::iter().map(write_asset).all(|success| success) {
            println!("Created a template at ./template.");
            println!("Created a config at ./config.dhall.");
            println!("See config for more information how setting up.");

            if Path::new("./output")
                .try_exists()
                .expect("Failed to check existence of ./output")
            {
                println!(concat!(
                    "WARNING: ./output already exists and would be overwritten ",
                    "unless you move it or modify gitja's output destination ",
                    "in config.dhall.",
                ))
            }
        } else {
            println!("There were some failures.");
        }
        true
    }
}

fn write_asset(name: Cow<'static, str>) -> bool {
    let asset = Assets::get(&name).unwrap();
    let mut buf = PathBuf::from(name.as_ref());

    // We copy templates/base/* from the source to template/* locally, so need to convert those
    // first parts of the paths.
    if let Ok(stripped) = buf.as_path().strip_prefix("templates/base") {
        buf = Path::new("template").join(stripped);
    }

    let path = buf.as_path();
    path.parent().and_then(|p| fs::create_dir_all(p).ok());

    if fs::write(path, &asset.data).is_err() {
        println!("Failed to write to {}", path.display());
        false
    } else {
        true
    }
}
