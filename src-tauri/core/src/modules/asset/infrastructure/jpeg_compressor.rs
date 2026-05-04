use crate::modules::asset::domain::image_compressor::ImageCompressor;
use crate::modules::asset::domain::optimization_error::OptimizationError;
use image::codecs::jpeg::JpegEncoder;
use image::load_from_memory;
use std::io::Cursor;

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
        let img = load_from_memory(input)
            .map_err(|e| OptimizationError::CompressionError(e.to_string()))?;

        let mut output = Vec::new();
        let mut cursor = Cursor::new(&mut output);

        let mut encoder = JpegEncoder::new_with_quality(&mut cursor, self.quality);
        encoder
            .encode_image(&img)
            .map_err(|e| OptimizationError::CompressionError(e.to_string()))?;

        Ok(output)
    }
}
