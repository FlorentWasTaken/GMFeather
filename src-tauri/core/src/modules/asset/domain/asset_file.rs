use crate::modules::asset::domain::asset_type::AssetType;
use std::path::PathBuf;

#[derive(Debug, Clone)]
pub struct AssetFile {
    pub path: PathBuf,
    pub asset_type: AssetType,
    pub original_size: u64,
}

impl AssetFile {
    pub const fn new(path: PathBuf, asset_type: AssetType, original_size: u64) -> Self {
        Self {
            path,
            asset_type,
            original_size,
        }
    }
}
