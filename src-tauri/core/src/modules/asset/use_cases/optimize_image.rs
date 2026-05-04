use crate::modules::asset::domain::asset_detector::AssetDetector;
use crate::modules::asset::domain::asset_type::AssetType;
use crate::modules::asset::domain::image_compressor::ImageCompressor;
use crate::modules::asset::domain::image_validator::ImageValidator;
use crate::modules::asset::domain::optimization_error::OptimizationError;
use crate::modules::asset::domain::optimization_options::OptimizationOptions;
use crate::modules::asset::domain::optimization_result::OptimizationResult;
use image::{load_from_memory, GenericImageView, ImageFormat};
use std::fs;
use std::io::Cursor;
use std::path::Path;
use tracing::info;

pub struct OptimizeImageUseCase<'a> {
    detector: &'a dyn AssetDetector,
    png_compressor: &'a dyn ImageCompressor,
    jpeg_compressor: &'a dyn ImageCompressor,
    validator: &'a dyn ImageValidator,
}

impl<'a> OptimizeImageUseCase<'a> {
    pub fn new(
        detector: &'a dyn AssetDetector,
        png_compressor: &'a dyn ImageCompressor,
        jpeg_compressor: &'a dyn ImageCompressor,
        validator: &'a dyn ImageValidator,
    ) -> Self {
        Self {
            detector,
            png_compressor,
            jpeg_compressor,
            validator,
        }
    }

    pub fn execute(
        &self,
        path: &Path,
        options: &OptimizationOptions,
    ) -> Result<OptimizationResult, OptimizationError> {
        let asset_type = self.detect_type(path)?;
        let compressor = self.select_compressor(asset_type)?;

        let original_data = fs::read(path)?;
        let original_size = original_data.len() as u64;

        let optimized_data = self.optimize_data(&original_data, options, asset_type, compressor)?;

        if optimized_data.len() as u64 >= original_size {
            info!(path = ?path, "Optimization skipped: no size reduction");
            return Err(OptimizationError::OptimizationIneffective);
        }

        self.validator.validate(&optimized_data)?;
        self.persist_result(path, original_size, optimized_data)
    }

    fn detect_type(&self, path: &Path) -> Result<AssetType, OptimizationError> {
        self.detector
            .detect(path)
            .map_err(|e| OptimizationError::UnsupportedType(format!("{:?}", e)))
    }

    fn select_compressor(
        &self,
        asset_type: AssetType,
    ) -> Result<&dyn ImageCompressor, OptimizationError> {
        match asset_type {
            AssetType::PNG => Ok(self.png_compressor),
            AssetType::JPG => Ok(self.jpeg_compressor),
            _ => Err(OptimizationError::UnsupportedType(asset_type.to_string())),
        }
    }

    fn optimize_data(
        &self,
        data: &[u8],
        options: &OptimizationOptions,
        asset_type: AssetType,
        compressor: &dyn ImageCompressor,
    ) -> Result<Vec<u8>, OptimizationError> {
        let resized = self.resize_if_needed(data, options, asset_type)?;
        compressor.compress(&resized)
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
            original = result.original_size,
            optimized = result.optimized_size,
            ratio = %format!("{:.2}%", result.compression_ratio()),
            "Image optimized successfully"
        );
    }

    fn resize_if_needed(
        &self,
        data: &[u8],
        options: &OptimizationOptions,
        asset_type: AssetType,
    ) -> Result<Vec<u8>, OptimizationError> {
        if options.max_width.is_none() && options.max_height.is_none() {
            return Ok(data.to_vec());
        }

        let img = load_from_memory(data)
            .map_err(|e| OptimizationError::CompressionError(format!("Failed to load: {}", e)))?;

        if !self.should_resize(&img, options) {
            return Ok(data.to_vec());
        }

        self.perform_resize(&img, options, asset_type)
    }

    fn should_resize(&self, img: &image::DynamicImage, options: &OptimizationOptions) -> bool {
        let (w, h) = img.dimensions();
        w > options.max_width.unwrap_or(w) || h > options.max_height.unwrap_or(h)
    }

    fn perform_resize(
        &self,
        img: &image::DynamicImage,
        options: &OptimizationOptions,
        asset_type: AssetType,
    ) -> Result<Vec<u8>, OptimizationError> {
        let (w, h) = img.dimensions();
        let target_w = options.max_width.unwrap_or(w);
        let target_h = options.max_height.unwrap_or(h);

        info!(from = %format!("{}x{}", w, h), to = %format!("{}x{}", target_w, target_h), "Resizing image");

        let resized = img.resize(target_w, target_h, image::imageops::FilterType::Lanczos3);
        let mut output = Vec::new();
        let format = self.get_format(asset_type)?;

        resized
            .write_to(&mut Cursor::new(&mut output), format)
            .map_err(|e| OptimizationError::CompressionError(format!("Write failed: {}", e)))?;

        Ok(output)
    }

    fn get_format(&self, asset_type: AssetType) -> Result<ImageFormat, OptimizationError> {
        match asset_type {
            AssetType::PNG => Ok(ImageFormat::Png),
            AssetType::JPG => Ok(ImageFormat::Jpeg),
            _ => Err(OptimizationError::UnsupportedType(asset_type.to_string())),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::modules::asset::infrastructure::default_image_validator::DefaultImageValidator;
    use crate::modules::asset::infrastructure::file_asset_detector::FileAssetDetector;
    use crate::modules::asset::infrastructure::jpeg_compressor::JpegCompressor;
    use crate::modules::asset::infrastructure::oxipng_compressor::OxipngCompressor;
    use image::{ImageFormat, Rgb, RgbImage};
    use tempfile::tempdir;

    #[test]
    fn test_optimize_png_success() {
        let dir = tempdir().unwrap();
        let path = dir.path().join("test.png");
        let mut img = RgbImage::new(100, 100);
        for x in 0..100 {
            for y in 0..100 {
                img.put_pixel(x, y, Rgb([x as u8, y as u8, (x + y) as u8]));
            }
        }
        img.save(&path).unwrap();

        let detector = FileAssetDetector::new();
        let png_comp = OxipngCompressor::new();
        let jpeg_comp = JpegCompressor::new(80);
        let validator = DefaultImageValidator::new();
        let use_case = OptimizeImageUseCase::new(&detector, &png_comp, &jpeg_comp, &validator);

        let options = OptimizationOptions::default();
        let result = use_case.execute(&path, &options);

        match result {
            Ok(res) => assert!(res.optimized_size < res.original_size),
            Err(OptimizationError::OptimizationIneffective) => (),
            Err(e) => panic!("Optimization failed: {:?}", e),
        }
    }

    #[test]
    fn test_optimize_jpg_success() {
        let dir = tempdir().unwrap();
        let path = dir.path().join("test.jpg");
        let mut img = RgbImage::new(100, 100);
        for x in 0..100 {
            for y in 0..100 {
                img.put_pixel(x, y, Rgb([x as u8, y as u8, (x + y) as u8]));
            }
        }
        img.save(&path).unwrap();

        let detector = FileAssetDetector::new();
        let png_comp = OxipngCompressor::new();
        let jpeg_comp = JpegCompressor::new(10);
        let validator = DefaultImageValidator::new();
        let use_case = OptimizeImageUseCase::new(&detector, &png_comp, &jpeg_comp, &validator);

        let options = OptimizationOptions::default();
        let result = use_case
            .execute(&path, &options)
            .expect("Should optimize JPG");
        assert!(result.optimized_size < result.original_size);
    }

    #[test]
    fn test_optimize_png_with_resize() {
        let dir = tempdir().unwrap();
        let path = dir.path().join("test_resize.png");
        let mut img = RgbImage::new(200, 200);
        for x in 0..200 {
            for y in 0..200 {
                img.put_pixel(x, y, Rgb([x as u8, y as u8, (x + y) as u8]));
            }
        }
        img.save(&path).unwrap();

        let detector = FileAssetDetector::new();
        let png_comp = OxipngCompressor::new();
        let jpeg_comp = JpegCompressor::new(80);
        let validator = DefaultImageValidator::new();
        let use_case = OptimizeImageUseCase::new(&detector, &png_comp, &jpeg_comp, &validator);

        let options = OptimizationOptions::new(Some(100), Some(100));
        let result = use_case
            .execute(&path, &options)
            .expect("Should resize and optimize");

        assert!(result.optimized_size < result.original_size);
        let final_img = image::open(&path).unwrap();
        assert_eq!(final_img.width(), 100);
        assert_eq!(final_img.height(), 100);
    }

    #[test]
    fn test_optimize_png_no_resize_needed() {
        let dir = tempdir().unwrap();
        let path = dir.path().join("test_no_resize.png");
        let img = RgbImage::new(50, 50);
        img.save(&path).unwrap();

        let detector = FileAssetDetector::new();
        let png_comp = OxipngCompressor::new();
        let jpeg_comp = JpegCompressor::new(80);
        let validator = DefaultImageValidator::new();
        let use_case = OptimizeImageUseCase::new(&detector, &png_comp, &jpeg_comp, &validator);

        let options = OptimizationOptions::new(Some(100), Some(100));
        let _ = use_case.execute(&path, &options);

        let final_img = image::open(&path).unwrap();
        assert_eq!(final_img.width(), 50);
        assert_eq!(final_img.height(), 50);
    }
}
