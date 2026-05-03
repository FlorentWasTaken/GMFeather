use crate::modules::asset::domain::asset_type::AssetType;
use std::io;
use std::path::Path;

#[derive(Debug)]
pub enum AssetError {
    IoError(io::Error),
    DetectionFailed(String),
}

pub trait AssetDetector {
    fn detect(&self, path: &Path) -> Result<AssetType, AssetError>;
}
