pub mod optimize;

use clap::Subcommand;
use optimize::OptimizeArgs;

#[derive(Subcommand)]
pub enum Commands {
    Optimize(OptimizeArgs),
}

pub fn handle_command(command: &Commands) {
    match command {
        Commands::Optimize(args) => optimize::execute(args),
    }
}
