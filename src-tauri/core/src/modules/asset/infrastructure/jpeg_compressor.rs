use crate::modules::asset::domain::errors::optimization_error::OptimizationError;
use crate::modules::asset::domain::ports::image_compressor::ImageCompressor;
use image::codecs::jpeg::JpegEncoder;
use image::load_from_memory;
use std::io::Cursor;
use tracing::error;

pub struct JpegCompressor {
    quality: u8,
}

impl JpegCompressor {
    pub const fn new(quality: u8) -> Self {
        Self { quality }
    }
}

impl ImageCompressor for JpegCompressor {
    fn compress(&self, input: &[u8]) -> Result<Vec<u8>, OptimizationError> {
        let img = load_from_memory(input).map_err(|e| {
            error!(error = %e, "Failed to load image for JPEG compression");
            OptimizationError::CompressionError(e.to_string())
        })?;

        let mut output = Vec::new();
        let mut cursor = Cursor::new(&mut output);

        let mut encoder = JpegEncoder::new_with_quality(&mut cursor, self.quality);
        encoder.encode_image(&img).map_err(|e| {
            error!(error = %e, "JPEG encoding failed");
            OptimizationError::CompressionError(e.to_string())
        })?;

        Ok(output)
    }
}
