use crate::modules::asset::domain::asset_detector::{AssetDetector, AssetError};
use crate::modules::asset::domain::asset_type::AssetType;
use std::fs::File;
use std::io::Read;
use std::path::Path;

pub struct FileAssetDetector;

impl FileAssetDetector {
    pub const fn new() -> Self {
        Self
    }

    fn check_magic_bytes(&self, path: &Path) -> Result<AssetType, AssetError> {
        let mut file = File::open(path).map_err(AssetError::IoError)?;
        let mut buffer = [0u8; 12];
        let bytes_read = file.read(&mut buffer).map_err(AssetError::IoError)?;

        if bytes_read >= 4 {
            // VTF: VTF\0
            if &buffer[0..4] == b"VTF\0" {
                return Ok(AssetType::VTF);
            }
            // WAV: RIFF...WAVE
            if &buffer[0..4] == b"RIFF" && bytes_read >= 12 && &buffer[8..12] == b"WAVE" {
                return Ok(AssetType::WAV);
            }
            // PNG: \x89PNG
            if &buffer[0..4] == b"\x89PNG" {
                return Ok(AssetType::PNG);
            }
        }

        if bytes_read >= 3 {
            // MP3: ID3
            if &buffer[0..3] == b"ID3" {
                return Ok(AssetType::MP3);
            }
            // JPG: \xff\xd8\xff
            if &buffer[0..3] == b"\xff\xd8\xff" {
                return Ok(AssetType::JPG);
            }
        }

        Ok(AssetType::Unknown)
    }
}

impl AssetDetector for FileAssetDetector {
    fn detect(&self, path: &Path) -> Result<AssetType, AssetError> {
        let extension = path
            .extension()
            .and_then(|ext| ext.to_str())
            .map(|ext| ext.to_lowercase());

        let asset_type = match extension.as_deref() {
            Some("vtf") => self.check_magic_bytes(path).unwrap_or(AssetType::VTF),
            Some("wav") => self.check_magic_bytes(path).unwrap_or(AssetType::WAV),
            Some("png") => self.check_magic_bytes(path).unwrap_or(AssetType::PNG),
            Some("jpg") | Some("jpeg") => self.check_magic_bytes(path).unwrap_or(AssetType::JPG),
            Some("mp3") => self.check_magic_bytes(path).unwrap_or(AssetType::MP3),
            Some("vmt") => AssetType::VMT,
            Some("lua") => AssetType::LUA,
            _ => self.check_magic_bytes(path)?,
        };

        Ok(asset_type)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;

    #[test]
    fn test_detect_vtf() {
        let detector = FileAssetDetector::new();
        let dir = tempfile::tempdir().unwrap();
        let file_path = dir.path().join("test.vtf");
        {
            let mut f = File::create(&file_path).unwrap();
            f.write_all(b"VTF\0").unwrap();
        }

        let result = detector.detect(&file_path).unwrap();
        assert_eq!(result, AssetType::VTF);
    }

    #[test]
    fn test_detect_wav() {
        let dir = tempfile::tempdir().unwrap();
        let file_path = dir.path().join("test.wav");
        {
            let mut f = File::create(&file_path).unwrap();
            f.write_all(b"RIFF\0\0\0\0WAVE").unwrap();
        }
        let detector = FileAssetDetector::new();
        let result = detector.detect(&file_path).unwrap();
        assert_eq!(result, AssetType::WAV);
    }

    #[test]
    fn test_detect_vmt() {
        let dir = tempfile::tempdir().unwrap();
        let file_path = dir.path().join("test.vmt");
        {
            let mut f = File::create(&file_path).unwrap();
            f.write_all(b"LightmappedGeneric").unwrap();
        }
        let detector = FileAssetDetector::new();
        let result = detector.detect(&file_path).unwrap();
        assert_eq!(result, AssetType::VMT);
    }

    #[test]
    fn test_detect_lua() {
        let dir = tempfile::tempdir().unwrap();
        let file_path = dir.path().join("test.lua");
        {
            let mut f = File::create(&file_path).unwrap();
            f.write_all(b"print('hello')").unwrap();
        }
        let detector = FileAssetDetector::new();
        let result = detector.detect(&file_path).unwrap();
        assert_eq!(result, AssetType::LUA);
    }

    #[test]
    fn test_detect_png() {
        let dir = tempfile::tempdir().unwrap();
        let file_path = dir.path().join("test.png");
        {
            let mut f = File::create(&file_path).unwrap();
            f.write_all(b"\x89PNG\r\n\x1a\n").unwrap();
        }
        let detector = FileAssetDetector::new();
        let result = detector.detect(&file_path).unwrap();
        assert_eq!(result, AssetType::PNG);
    }

    #[test]
    fn test_detect_jpg() {
        let dir = tempfile::tempdir().unwrap();
        let file_path = dir.path().join("test.jpg");
        {
            let mut f = File::create(&file_path).unwrap();
            f.write_all(b"\xff\xd8\xff").unwrap();
        }
        let detector = FileAssetDetector::new();
        let result = detector.detect(&file_path).unwrap();
        assert_eq!(result, AssetType::JPG);
    }

    #[test]
    fn test_detect_mp3() {
        let dir = tempfile::tempdir().unwrap();
        let file_path = dir.path().join("test.mp3");
        {
            let mut f = File::create(&file_path).unwrap();
            f.write_all(b"ID3\x03\x00\x00\x00").unwrap();
        }
        let detector = FileAssetDetector::new();
        let result = detector.detect(&file_path).unwrap();
        assert_eq!(result, AssetType::MP3);
    }

    #[test]
    fn test_detect_unknown() {
        let dir = tempfile::tempdir().unwrap();
        let file_path = dir.path().join("test.txt");
        {
            let mut f = File::create(&file_path).unwrap();
            f.write_all(b"random text").unwrap();
        }
        let detector = FileAssetDetector::new();
        let result = detector.detect(&file_path).unwrap();
        assert_eq!(result, AssetType::Unknown);
    }
}
