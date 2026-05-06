use clap::Args;
use feather_core::modules::asset::domain::optimization_options::OptimizationOptions;
use feather_core::modules::asset::infrastructure::default_image_validator::DefaultImageValidator;
use feather_core::modules::asset::infrastructure::file_asset_detector::FileAssetDetector;
use feather_core::modules::asset::infrastructure::file_backup_service::FileBackupService;
use feather_core::modules::asset::infrastructure::jpeg_compressor::JpegCompressor;
use feather_core::modules::asset::infrastructure::oxipng_compressor::OxipngCompressor;
use feather_core::modules::asset::use_cases::optimize_image::OptimizeImageUseCase;
use std::path::Path;
use tracing::{error, info, warn};
use walkdir::WalkDir;

#[derive(Args)]
pub struct OptimizeArgs {
    #[arg(help = "Path to the directory or file to optimize")]
    pub path: String,

    #[arg(long, help = "Simulate the process without modifying any files")]
    pub dry_run: bool,

    #[arg(
        long,
        help = "Maximum width for resized images (maintains aspect ratio)"
    )]
    pub max_width: Option<u32>,

    #[arg(
        long,
        help = "Maximum height for resized images (maintains aspect ratio)"
    )]
    pub max_height: Option<u32>,

    #[arg(long, help = "Maximum recursion depth for directory scanning")]
    pub max_depth: Option<usize>,

    #[arg(long, help = "Skip creating .bak files (not recommended)")]
    pub no_backup: bool,
}

pub fn execute(args: &OptimizeArgs) {
    let path = Path::new(&args.path);
    if !path.exists() {
        error!("Path does not exist: {}", args.path);
        return;
    }

    let detector = FileAssetDetector::new();
    let png_comp = OxipngCompressor::new();
    let jpeg_comp = JpegCompressor::new(80);
    let validator = DefaultImageValidator::new();
    let backup = FileBackupService::new();
    let use_case = OptimizeImageUseCase::new(&detector, &png_comp, &jpeg_comp, &validator, &backup);

    process_path(path, args, &use_case);
}

fn process_path(path: &Path, args: &OptimizeArgs, use_case: &OptimizeImageUseCase) {
    let options = OptimizationOptions::new(args.max_width, args.max_height, !args.no_backup);

    if path.is_file() {
        optimize_file(path, args.dry_run, use_case, &options);
        return;
    }

    let mut walker = WalkDir::new(path);
    if let Some(depth) = args.max_depth {
        walker = walker.max_depth(depth);
    }

    for entry in walker.into_iter().filter_map(|e| e.ok()) {
        if entry.file_type().is_file() {
            optimize_file(entry.path(), args.dry_run, use_case, &options);
        }
    }
}

fn optimize_file(
    path: &Path,
    dry_run: bool,
    use_case: &OptimizeImageUseCase,
    options: &OptimizationOptions,
) {
    if dry_run {
        info!("Dry run: would optimize {:?} with {:?}", path, options);
        return;
    }

    match use_case.execute(path, options) {
        Ok(_) => info!("Optimized {:?}", path),
        Err(e) => warn!("Skipped/Failed {:?}: {:?}", path, e),
    }
}
