use crate::common::formatter::format_size;
use crate::modules::asset::domain::errors::optimization_error::OptimizationError;
use crate::modules::asset::domain::models::asset_type::AssetType;
use crate::modules::asset::domain::models::optimization_options::OptimizationOptions;
use crate::modules::asset::domain::models::optimization_result::OptimizationResult;
use crate::modules::asset::domain::ports::asset_detector::AssetDetector;
use crate::modules::asset::domain::ports::audio_compressor::AudioCompressor;
use crate::modules::asset::domain::ports::backup_service::BackupService;
use std::fs;
use std::path::Path;
use tracing::info;

pub struct OptimizeAudioUseCase<'a> {
    detector: &'a dyn AssetDetector,
    wav_compressor: &'a dyn AudioCompressor,
    mp3_compressor: &'a dyn AudioCompressor,
    backup_service: &'a dyn BackupService,
}

impl<'a> OptimizeAudioUseCase<'a> {
    pub fn new(
        detector: &'a dyn AssetDetector,
        wav_compressor: &'a dyn AudioCompressor,
        mp3_compressor: &'a dyn AudioCompressor,
        backup_service: &'a dyn BackupService,
    ) -> Self {
        Self {
            detector,
            wav_compressor,
            mp3_compressor,
            backup_service,
        }
    }

    pub fn execute(
        &self,
        path: &Path,
        options: &OptimizationOptions,
    ) -> Result<OptimizationResult, OptimizationError> {
        let (_asset_type, compressor) = self.prepare_context(path)?;
        let original_data = fs::read(path)?;
        let original_size = original_data.len() as u64;

        let target_rate = options.target_sample_rate.unwrap_or(44100);
        let optimized_data = compressor.compress(&original_data, target_rate)?;

        self.ensure_improvement(path, original_size, optimized_data.len() as u64)?;

        if options.create_backup {
            self.backup_service.backup(path)?;
        }

        self.persist_result(path, original_size, optimized_data)
    }

    fn prepare_context(
        &self,
        path: &Path,
    ) -> Result<(AssetType, &dyn AudioCompressor), OptimizationError> {
        let asset_type = self.detect_type(path)?;
        let compressor = self.select_compressor(asset_type)?;
        Ok((asset_type, compressor))
    }

    fn ensure_improvement(
        &self,
        path: &Path,
        original: u64,
        optimized: u64,
    ) -> Result<(), OptimizationError> {
        if optimized >= original {
            info!(path = ?path, "Optimization skipped: no size reduction");
            return Err(OptimizationError::OptimizationIneffective);
        }
        Ok(())
    }

    fn detect_type(&self, path: &Path) -> Result<AssetType, OptimizationError> {
        self.detector
            .detect(path)
            .map_err(|e| OptimizationError::UnsupportedType(format!("{:?}", e)))
    }

    fn select_compressor(
        &self,
        asset_type: AssetType,
    ) -> Result<&dyn AudioCompressor, OptimizationError> {
        match asset_type {
            AssetType::WAV => Ok(self.wav_compressor),
            AssetType::MP3 => Ok(self.mp3_compressor),
            _ => Err(OptimizationError::UnsupportedType(asset_type.to_string())),
        }
    }

    fn persist_result(
        &self,
        path: &Path,
        original_size: u64,
        data: Vec<u8>,
    ) -> Result<OptimizationResult, OptimizationError> {
        let optimized_size = data.len() as u64;
        fs::write(path, &data)?;

        let result = OptimizationResult::new(path.to_path_buf(), original_size, optimized_size);
        self.log_success(path, &result);
        Ok(result)
    }

    fn log_success(&self, path: &Path, result: &OptimizationResult) {
        info!(
            path = ?path,
            original = %format_size(result.original_size),
            optimized = %format_size(result.optimized_size),
            ratio = %format!("{:.2}%", result.compression_ratio()),
            "Audio optimized successfully"
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::modules::asset::infrastructure::file_asset_detector::FileAssetDetector;
    use crate::modules::asset::infrastructure::file_backup_service::FileBackupService;
    use crate::modules::asset::infrastructure::mp3_compressor::Mp3Compressor;
    use crate::modules::asset::infrastructure::wav_compressor::WavCompressor;
    use tempfile::tempdir;

    fn create_test_wav() -> Vec<u8> {
        let sample_rate: u32 = 48000;
        let channels: u16 = 1;
        let data = vec![0.0f32; 48000];
        let mut wav = Vec::new();
        wav.extend_from_slice(b"RIFF");
        wav.extend_from_slice(&((36 + data.len() * 2) as u32).to_le_bytes());
        wav.extend_from_slice(b"WAVEfmt ");
        wav.extend_from_slice(&16u32.to_le_bytes());
        wav.extend_from_slice(&1u16.to_le_bytes());
        wav.extend_from_slice(&(channels as u16).to_le_bytes());
        wav.extend_from_slice(&sample_rate.to_le_bytes());
        wav.extend_from_slice(&(sample_rate * channels as u32 * 2).to_le_bytes());
        wav.extend_from_slice(&(channels as u16 * 2).to_le_bytes());
        wav.extend_from_slice(&16u16.to_le_bytes());
        wav.extend_from_slice(b"data");
        wav.extend_from_slice(&((data.len() * 2) as u32).to_le_bytes());
        for &s in &data {
            let scaled = (s * 32767.0).clamp(-32768.0, 32767.0) as i16;
            wav.extend_from_slice(&scaled.to_le_bytes());
        }
        wav
    }

    #[test]
    fn test_optimize_wav_success() {
        let dir = tempdir().unwrap();
        let path = dir.path().join("test.wav");
        let wav_data = create_test_wav();
        std::fs::write(&path, &wav_data).unwrap();

        let detector = FileAssetDetector::new();
        let wav_comp = WavCompressor::new();
        let mp3_comp = Mp3Compressor::new();
        let backup = FileBackupService::new();
        let use_case = OptimizeAudioUseCase::new(&detector, &wav_comp, &mp3_comp, &backup);

        let options = OptimizationOptions::new(None, None, false, Some(22050));
        let result = use_case.execute(&path, &options).unwrap();

        assert!(result.optimized_size < result.original_size);
    }
}
