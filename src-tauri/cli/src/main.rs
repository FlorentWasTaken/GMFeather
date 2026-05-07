use clap::Parser;
use feather_core::infrastructure::config;
use tracing::Level;
use tracing_subscriber::{fmt, prelude::*, EnvFilter};

mod commands;

use commands::{handle_command, Commands};

#[derive(Parser)]
#[command(name = "gmfeather")]
#[command(author, version, about = "Garry's Mod Asset Optimization Suite", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,

    #[arg(short, long, global = true)]
    verbose: bool,
}

fn main() {
    let cli = Cli::parse();
    setup_logging(cli.verbose);
    config::init_config();
    handle_command(&cli.command);
}

fn setup_logging(verbose: bool) {
    let log_level = if verbose { Level::DEBUG } else { Level::INFO };

    tracing_subscriber::registry()
        .with(fmt::layer())
        .with(EnvFilter::from_default_env().add_directive(log_level.into()))
        .init();
}
