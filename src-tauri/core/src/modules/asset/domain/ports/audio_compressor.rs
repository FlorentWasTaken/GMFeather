use crate::modules::asset::domain::errors::optimization_error::OptimizationError;

pub trait AudioCompressor {
    fn decode(&self, input: &[u8]) -> Result<(Vec<f32>, u32, u16), OptimizationError>;
    fn resample(&self, samples: &[f32], from: u32, to: u32, channels: u16) -> Vec<f32>;
    fn encode(
        &self,
        pcm: &[f32],
        sample_rate: u32,
        channels: u16,
        bitrate: Option<u32>,
    ) -> Result<Vec<u8>, OptimizationError>;
}
