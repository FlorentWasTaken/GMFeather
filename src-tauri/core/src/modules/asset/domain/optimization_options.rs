#[derive(Debug, Clone, Default)]
pub struct OptimizationOptions {
    pub max_width: Option<u32>,
    pub max_height: Option<u32>,
}

impl OptimizationOptions {
    pub const fn new(max_width: Option<u32>, max_height: Option<u32>) -> Self {
        Self {
            max_width,
            max_height,
        }
    }
}
