use crate::modules::asset::domain::errors::optimization_error::OptimizationError;
use crate::modules::asset::domain::ports::image_validator::ImageValidator;
use image::load_from_memory;

pub struct DefaultImageValidator;

impl Default for DefaultImageValidator {
    fn default() -> Self {
        Self::new()
    }
}

impl DefaultImageValidator {
    pub const fn new() -> Self {
        Self
    }
}

impl ImageValidator for DefaultImageValidator {
    fn validate(&self, data: &[u8]) -> Result<(), OptimizationError> {
        load_from_memory(data)
            .map(|_| ())
            .map_err(|e| OptimizationError::ValidationError(format!("Corrupted image data: {}", e)))
    }
}
