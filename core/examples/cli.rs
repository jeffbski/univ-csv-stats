use anyhow::{Context, Result};
use clap::Parser;
use std::path::PathBuf;
use univ_csv_stats_core::calculate_stats_from_file;

/// A simple CLI to calculate statistics from a CSV file.
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// The path to the CSV file to process
    #[arg()]
    file_path: PathBuf,
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    let file_path = &cli.file_path;

    // Convert the PathBuf to a &str, returning an error if it's not valid UTF-8.
    let file_path_str = file_path
        .to_str()
        .context("The provided file path is not valid UTF-8")?;

    println!("Calculating statistics for '{file_path_str}'...");

    // Call the core logic. The '?' operator will propagate any errors, which will be
    // automatically formatted and printed to stderr by anyhow.
    let stats = calculate_stats_from_file(file_path_str)
        .with_context(|| format!("Failed to process the CSV file '{file_path_str}'"))?;

    // If successful, print the statistics.
    println!("\nOutput for rust-core");
    println!("Count:              {}", stats.count);
    println!("Min:                {:.4}", stats.min);
    println!("Max:                {:.4}", stats.max);
    println!("Sum:                {:.4}", stats.sum);
    println!("Mean:               {:.4}", stats.mean);
    println!("Variance:           {:.4}", stats.variance);
    println!("Standard Deviation: {:.4}", stats.standard_deviation);
    println!("Skewness:           {:.4}", stats.skewness);
    println!("Kurtosis:           {:.4}", stats.kurtosis);
    println!("----------------------------------------");

    Ok(())
}
