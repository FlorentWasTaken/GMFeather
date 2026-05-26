use crate::modules::asset::domain::errors::optimization_error::OptimizationError;

pub trait AudioCompressor {
    fn compress(&self, input: &[u8], target_sample_rate: u32)
        -> Result<Vec<u8>, OptimizationError>;
}
