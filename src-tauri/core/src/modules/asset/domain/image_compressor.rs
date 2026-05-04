use crate::modules::asset::domain::optimization_error::OptimizationError;

pub trait ImageCompressor {
    fn compress(&self, input: &[u8]) -> Result<Vec<u8>, OptimizationError>;
}
