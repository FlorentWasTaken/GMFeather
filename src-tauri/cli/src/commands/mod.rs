pub mod clean;
pub mod optimize;
pub mod rollback;

use clap::Subcommand;
use clean::CleanArgs;
use optimize::OptimizeArgs;
use rollback::RollbackArgs;

#[derive(Subcommand)]
pub enum Commands {
    Optimize(OptimizeArgs),
    Rollback(RollbackArgs),
    Clean(CleanArgs),
}

pub fn handle_command(command: &Commands) {
    match command {
        Commands::Optimize(args) => optimize::execute(args),
        Commands::Rollback(args) => rollback::execute(args),
        Commands::Clean(args) => clean::execute(args),
    }
}
