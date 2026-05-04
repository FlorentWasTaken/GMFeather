use crate::modules::asset::domain::image_compressor::ImageCompressor;
use crate::modules::asset::domain::optimization_error::OptimizationError;
use oxipng::{optimize_from_memory, Options};
use tracing::{debug, error};

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
        debug!(input_size = input.len(), "Starting oxipng compression");

        optimize_from_memory(input, &Options::default()).map_err(|e| {
            error!(error = %e, "Oxipng compression failed");
            OptimizationError::CompressionError(e.to_string())
        })
    }
}
