use crate::modules::asset::domain::image_compressor::ImageCompressor;
use crate::modules::asset::domain::optimization_error::OptimizationError;
use oxipng::{optimize_from_memory, Options};

pub struct OxipngCompressor;

impl Default for OxipngCompressor {
    fn default() -> Self {
        Self::new()
    }
}

impl OxipngCompressor {
    pub const fn new() -> Self {
        Self
    }
}

impl ImageCompressor for OxipngCompressor {
    fn compress(&self, input: &[u8]) -> Result<Vec<u8>, OptimizationError> {
        let options = Options::default();
        optimize_from_memory(input, &options)
            .map_err(|e| OptimizationError::CompressionError(e.to_string()))
    }
}
