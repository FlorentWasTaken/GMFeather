use crate::modules::asset::domain::models::asset_type::AssetType;

#[derive(Debug, Clone, Default)]
pub struct OptimizationOptions {
    pub max_width: Option<u32>,
    pub max_height: Option<u32>,
    pub create_backup: bool,
    pub target_sample_rate: Option<u32>,
    pub target_audio_format: Option<AssetType>,
    pub target_bitrate: Option<u32>,
}

impl OptimizationOptions {
    pub const fn new(
        max_width: Option<u32>,
        max_height: Option<u32>,
        create_backup: bool,
        target_sample_rate: Option<u32>,
        target_audio_format: Option<AssetType>,
        target_bitrate: Option<u32>,
    ) -> Self {
        Self {
            max_width,
            max_height,
            create_backup,
            target_sample_rate,
            target_audio_format,
            target_bitrate,
        }
    }
}
