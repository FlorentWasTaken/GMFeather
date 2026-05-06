use clap::Args;
use feather_core::modules::asset::infrastructure::file_backup_service::FileBackupService;
use feather_core::modules::asset::use_cases::rollback_asset::RollbackAssetUseCase;
use std::path::Path;
use tracing::{error, info};
use walkdir::WalkDir;

#[derive(Args)]
pub struct RollbackArgs {
    pub path: String,

    #[arg(long)]
    pub max_depth: Option<usize>,
}

pub fn execute(args: &RollbackArgs) {
    let path = Path::new(&args.path);
    if !path.exists() {
        error!("Path does not exist: {}", args.path);
        return;
    }

    let backup = FileBackupService::new();
    let use_case = RollbackAssetUseCase::new(&backup);

    process_path(path, args, &use_case);
}

fn process_path(path: &Path, args: &RollbackArgs, use_case: &RollbackAssetUseCase) {
    if path.is_file() {
        rollback_file(path, use_case);
        return;
    }

    let mut walker = WalkDir::new(path);
    if let Some(depth) = args.max_depth {
        walker = walker.max_depth(depth);
    }

    for entry in walker.into_iter().filter_map(|e| e.ok()) {
        if entry.file_type().is_file() {
            rollback_file(entry.path(), use_case);
        }
    }
}

fn rollback_file(path: &Path, use_case: &RollbackAssetUseCase) {
    match use_case.execute(path) {
        Ok(_) => info!("Restored {:?}", path),
        Err(_) => (),
    }
}
