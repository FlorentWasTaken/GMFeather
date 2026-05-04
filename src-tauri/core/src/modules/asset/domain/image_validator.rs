use crate::modules::asset::domain::optimization_error::OptimizationError;

pub trait ImageValidator {
    fn validate(&self, data: &[u8]) -> Result<(), OptimizationError>;
}
