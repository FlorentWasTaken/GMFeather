pub mod clean;
pub mod optimize;
pub mod rollback;

use clap::Subcommand;
use clean::CleanArgs;
use optimize::OptimizeArgs;
use rollback::RollbackArgs;

#[derive(Subcommand)]
pub enum Commands {
    #[command(about = "Optimize assets (images, textures) to reduce size and VRAM usage")]
    Optimize(OptimizeArgs),

    #[command(about = "Restore original assets from AppData backups")]
    Rollback(RollbackArgs),

    #[command(about = "Delete backups for a specific path to free up space")]
    Clean(CleanArgs),
}

pub fn handle_command(command: &Commands) {
    match command {
        Commands::Optimize(args) => optimize::execute(args),
        Commands::Rollback(args) => rollback::execute(args),
        Commands::Clean(args) => clean::execute(args),
    }
}
