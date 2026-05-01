use clap::Parser;
use feather_core::infrastructure::config;
use feather_core::use_cases;
use tracing::info;
use tracing_subscriber::{fmt, prelude::*, EnvFilter};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    name: String,
}

fn main() {
    tracing_subscriber::registry()
        .with(fmt::layer())
        .with(EnvFilter::from_default_env().add_directive(tracing::Level::INFO.into()))
        .init();

    config::init_config();

    let args = Args::parse();
    info!("Starting GMFeather CLI...");

    let message = use_cases::greet_user(&args.name);
    println!("{}", message);
}
