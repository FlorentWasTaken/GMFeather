use crate::modules::asset::domain::errors::optimization_error::OptimizationError;
use std::path::Path;

pub trait BackupService: Send + Sync {
    fn backup(&self, path: &Path) -> Result<(), OptimizationError>;
    fn restore(&self, path: &Path) -> Result<(), OptimizationError>;
    fn clear(&self, path: &Path) -> Result<(), OptimizationError>;
}
