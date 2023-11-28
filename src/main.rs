use std::path::PathBuf;

use clap::{Parser, Subcommand};

mod config;
mod init;
mod run;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Debug, Subcommand)]
enum Commands {
    /// Create a basic template with config in the current folder.
    #[command()]
    Init,

    /// Run gitja
    #[command()]
    Run {
        /// Use the specified config file.
        #[arg(short, long, default_value = "config.dhall", value_name = "FILE")]
        config: PathBuf,

        /// Suppress non-error output.
        #[arg(short, long, default_value = "false")]
        quiet: bool,

        /// Force regeneration of all files.
        #[arg(short, long, default_value = "false")]
        force: bool,
    },
}

fn main() {
    let args = Cli::parse();

    match args.command {
        Commands::Init => {
            init::init();
        }
        Commands::Run {
            config,
            quiet,
            force,
        } => {
            run::run(config, quiet, force);
        }
    }
}
