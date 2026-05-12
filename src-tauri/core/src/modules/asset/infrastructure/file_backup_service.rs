use crate::modules::asset::domain::errors::optimization_error::OptimizationError;
use crate::modules::asset::domain::ports::backup_service::BackupService;
use directories::ProjectDirs;
use std::fs;
use std::path::{Component, Path, PathBuf};

pub struct FileBackupService {
    backup_root: PathBuf,
}

impl FileBackupService {
    pub fn new() -> Self {
        Self::default()
    }

    #[cfg(test)]
    pub fn with_root(backup_root: PathBuf) -> Self {
        Self { backup_root }
    }

    fn get_backup_path(&self, path: &Path) -> PathBuf {
        let mut backup_path = self.backup_root.clone();

        for component in path.components() {
            match component {
                Component::Prefix(prefix) => {
                    let p = prefix.as_os_str().to_string_lossy().replace(':', "");
                    backup_path.push(p);
                }
                Component::RootDir => {}
                Component::Normal(p) => backup_path.push(p),
                _ => {}
            }
        }
        backup_path
    }
}

impl Default for FileBackupService {
    fn default() -> Self {
        let proj_dirs = ProjectDirs::from("com", "florentwastaken", "gmfeather")
            .expect("Could not determine AppData directory");
        let backup_root = proj_dirs.data_local_dir().join("backups");
        Self { backup_root }
    }
}

impl BackupService for FileBackupService {
    fn backup(&self, path: &Path) -> Result<(), OptimizationError> {
        let backup_path = self.get_backup_path(path);
        if let Some(parent) = backup_path.parent() {
            fs::create_dir_all(parent)?;
        }
        fs::copy(path, backup_path)?;
        Ok(())
    }

    fn restore(&self, path: &Path) -> Result<(), OptimizationError> {
        let backup_path = self.get_backup_path(path);
        if !backup_path.exists() {
            return Err(OptimizationError::IoError(std::io::Error::new(
                std::io::ErrorKind::NotFound,
                "Backup file not found in AppData",
            )));
        }
        fs::copy(&backup_path, path)?;
        Ok(())
    }

    fn clear(&self, path: &Path) -> Result<(), OptimizationError> {
        let backup_path = self.get_backup_path(path);
        if backup_path.exists() {
            if backup_path.is_file() {
                fs::remove_file(backup_path)?;
            } else {
                fs::remove_dir_all(backup_path)?;
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[test]
    fn test_backup_and_restore_success() {
        let asset_dir = tempdir().unwrap();
        let backup_dir = tempdir().unwrap();

        let asset_path = asset_dir.path().join("test.txt");
        fs::write(&asset_path, "original content").unwrap();

        let service = FileBackupService::with_root(backup_dir.path().to_path_buf());

        service.backup(&asset_path).expect("Backup should succeed");

        fs::write(&asset_path, "new content").unwrap();

        service
            .restore(&asset_path)
            .expect("Restore should succeed");

        let restored_content = fs::read_to_string(&asset_path).unwrap();
        assert_eq!(restored_content, "original content");
    }

    #[test]
    fn test_clear_backups() {
        let asset_dir = tempdir().unwrap();
        let backup_dir = tempdir().unwrap();

        let asset_path = asset_dir.path().join("to_clean.txt");
        fs::write(&asset_path, "content").unwrap();

        let service = FileBackupService::with_root(backup_dir.path().to_path_buf());
        service.backup(&asset_path).unwrap();

        let backup_path = service.get_backup_path(&asset_path);
        assert!(backup_path.exists());

        service.clear(&asset_path).expect("Clear should succeed");
        assert!(!backup_path.exists());
    }
}
