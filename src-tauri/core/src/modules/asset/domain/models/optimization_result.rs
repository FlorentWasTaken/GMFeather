use std::path::PathBuf;

#[derive(Debug, Clone)]
pub struct OptimizationResult {
    pub path: PathBuf,
    pub original_size: u64,
    pub optimized_size: u64,
}

impl OptimizationResult {
    pub const fn new(path: PathBuf, original_size: u64, optimized_size: u64) -> Self {
        Self {
            path,
            original_size,
            optimized_size,
        }
    }

    pub fn compression_ratio(&self) -> f64 {
        if self.original_size == 0 {
            return 0.0;
        }
        (1.0 - (self.optimized_size as f64 / self.original_size as f64)) * 100.0
    }
}
