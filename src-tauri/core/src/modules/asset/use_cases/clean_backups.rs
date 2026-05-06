use crate::modules::asset::domain::backup_service::BackupService;
use crate::modules::asset::domain::optimization_error::OptimizationError;
use std::path::Path;

pub struct CleanBackupsUseCase<'a> {
    backup_service: &'a dyn BackupService,
}

impl<'a> CleanBackupsUseCase<'a> {
    pub fn new(backup_service: &'a dyn BackupService) -> Self {
        Self { backup_service }
    }

    pub fn execute(&self, path: &Path) -> Result<(), OptimizationError> {
        self.backup_service.clear(path)
    }
}
