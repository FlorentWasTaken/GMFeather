use clap::Args;
use feather_core::modules::asset::infrastructure::file_backup_service::FileBackupService;
use feather_core::modules::asset::use_cases::clean_backups::CleanBackupsUseCase;
use std::path::Path;
use tracing::{error, info};

#[derive(Args)]
pub struct CleanArgs {
    pub path: String,
}

pub fn execute(args: &CleanArgs) {
    let path = Path::new(&args.path);
    let backup = FileBackupService::new();
    let use_case = CleanBackupsUseCase::new(&backup);

    match use_case.execute(path) {
        Ok(_) => info!("Backups cleared for {:?}", path),
        Err(e) => error!("Failed to clear backups: {:?}", e),
    }
}
