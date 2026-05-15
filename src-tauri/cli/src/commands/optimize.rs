use crate::ui::progress::{create_progress_bar, display_summary, BatchStats};
use clap::Args;
use feather_core::modules::asset::domain::models::optimization_options::OptimizationOptions;
use feather_core::modules::asset::infrastructure::default_image_validator::DefaultImageValidator;
use feather_core::modules::asset::infrastructure::file_asset_detector::FileAssetDetector;
use feather_core::modules::asset::infrastructure::file_backup_service::FileBackupService;
use feather_core::modules::asset::infrastructure::jpeg_compressor::JpegCompressor;
use feather_core::modules::asset::infrastructure::oxipng_compressor::OxipngCompressor;
use feather_core::modules::asset::use_cases::optimize_image::OptimizeImageUseCase;
use indicatif::ProgressBar;
use std::path::Path;
use tracing::error;
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

    let mut stats = BatchStats::default();
    process_with_feedback(path, args, &use_case, &mut stats);
    display_summary(&stats);
}

fn process_with_feedback(
    path: &Path,
    args: &OptimizeArgs,
    use_case: &OptimizeImageUseCase,
    stats: &mut BatchStats,
) {
    let total = if path.is_file() {
        1
    } else {
        count_files(path, args.max_depth)
    };
    let pb = create_progress_bar(total);
    process_path(path, args, use_case, stats, &pb);
    pb.finish_and_clear();
}

fn process_path(
    path: &Path,
    args: &OptimizeArgs,
    use_case: &OptimizeImageUseCase,
    stats: &mut BatchStats,
    pb: &ProgressBar,
) {
    let options = OptimizationOptions::new(args.max_width, args.max_height, !args.no_backup);
    if path.is_file() {
        process_single_file(path, args.dry_run, use_case, &options, stats, pb);
    } else {
        process_directory(path, args, use_case, &options, stats, pb);
    }
}

fn process_single_file(
    path: &Path,
    dry_run: bool,
    use_case: &OptimizeImageUseCase,
    options: &OptimizationOptions,
    stats: &mut BatchStats,
    pb: &ProgressBar,
) {
    optimize_file(path, dry_run, use_case, options, stats);
    pb.inc(1);
}

fn process_directory(
    path: &Path,
    args: &OptimizeArgs,
    use_case: &OptimizeImageUseCase,
    options: &OptimizationOptions,
    stats: &mut BatchStats,
    pb: &ProgressBar,
) {
    let mut walker = WalkDir::new(path);
    if let Some(depth) = args.max_depth {
        walker = walker.max_depth(depth);
    }
    for entry in walker
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.file_type().is_file())
    {
        process_single_file(entry.path(), args.dry_run, use_case, options, stats, pb);
    }
}

fn optimize_file(
    path: &Path,
    dry_run: bool,
    use_case: &OptimizeImageUseCase,
    options: &OptimizationOptions,
    stats: &mut BatchStats,
) {
    if dry_run {
        stats.files_processed += 1;
        return;
    }

    match use_case.execute(path, options) {
        Ok(res) => {
            stats.files_processed += 1;
            stats.space_saved += res.original_size.saturating_sub(res.optimized_size);
        }
        Err(feather_core::modules::asset::domain::errors::optimization_error::OptimizationError::OptimizationIneffective) => {
            stats.files_processed += 1;
        }
        Err(feather_core::modules::asset::domain::errors::optimization_error::OptimizationError::UnsupportedType(_)) => {
            stats.files_skipped += 1;
        }
        Err(_) => stats.errors += 1,
    }
}

fn count_files(path: &Path, max_depth: Option<usize>) -> u64 {
    let mut walker = WalkDir::new(path);
    if let Some(depth) = max_depth {
        walker = walker.max_depth(depth);
    }
    walker
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.file_type().is_file())
        .count() as u64
}
