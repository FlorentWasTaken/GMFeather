use comfy_table::{Cell, Color, Table};
use feather_core::common::formatter::format_size;
use indicatif::{ProgressBar, ProgressStyle};

#[derive(Default)]
pub struct BatchStats {
    pub files_processed: u64,
    pub files_skipped: u64,
    pub space_saved: u64,
    pub errors: u64,
}

pub fn create_progress_bar(total: u64) -> ProgressBar {
    let pb = ProgressBar::new(total);
    pb.set_style(
        ProgressStyle::default_bar()
            .template(
                "{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {pos}/{len} ({eta})",
            )
            .unwrap()
            .progress_chars("#>-"),
    );
    pb
}

pub fn display_summary(stats: &BatchStats) {
    let mut table = Table::new();
    table.set_header(vec!["Metric", "Value"]);
    table.add_row(vec!["Files processed", &stats.files_processed.to_string()]);
    table.add_row(vec!["Files skipped", &stats.files_skipped.to_string()]);
    table.add_row(vec![
        Cell::new("Space saved").fg(Color::Green),
        Cell::new(format_size(stats.space_saved)).fg(Color::Green),
    ]);
    table.add_row(vec![
        Cell::new("Errors").fg(Color::Red),
        Cell::new(stats.errors.to_string()).fg(Color::Red),
    ]);
    println!("\n{}", table);
}
