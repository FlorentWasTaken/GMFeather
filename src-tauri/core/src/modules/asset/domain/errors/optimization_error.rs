use std::io;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum OptimizationError {
    #[error("IO error: {0}")]
    IoError(#[from] io::Error),

    #[error("Compression failed: {0}")]
    CompressionError(String),

    #[error("Validation failed: {0}")]
    ValidationError(String),

    #[error("Unsupported asset type: {0}")]
    UnsupportedType(String),

    #[error("Optimized file is larger than original")]
    OptimizationIneffective,
}
