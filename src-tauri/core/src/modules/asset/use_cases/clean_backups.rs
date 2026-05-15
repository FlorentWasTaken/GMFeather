use crate::modules::asset::domain::errors::optimization_error::OptimizationError;
use crate::modules::asset::domain::ports::backup_service::BackupService;
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
