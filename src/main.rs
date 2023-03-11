use std::path::{Path, PathBuf};
use std::process::ExitCode;

use clap::Parser;

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

fn init() -> bool {
    if ["./template", "./config.dhall"]
        .iter()
        .map(Path::new)
        .filter(|p| p.try_exists().expect("message"))
        .map(|p| println!("Found path: {}", p.display()))
        .fold(false, |_, _| true)
    {
        println!("Please remove to continue.");
        false
    } else {
        println!("Did stuff...");
        // 3. Copy template from `templates/base` to `./template` *
        // 4. Copy `config.dhall` to `./config.dhall *
        // 5. Print a helpful message informing user of new files.
        // 6. If `./output` exists, warn user that running gitja immediately will overwrite that
        //    folder.
        // * embed assets into binary
        true
    }
}
