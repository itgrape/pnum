mod cli;

use anyhow::{Result, anyhow};
use clap::Parser;
use cli::{Args, SortKey};
use console::{StyledObject, style};
use indicatif::{ProgressBar, ProgressStyle};
use pnum::{Config, count_extensions};
use rayon::ThreadPoolBuilder;
use serde_json;
use std::time::Duration;

fn main() -> Result<()> {
    let args = Args::parse();
    ThreadPoolBuilder::new()
        .num_threads(args.thread_num as usize)
        .build_global()
        .map_err(|e| anyhow!("Failed to build thread pool: {}", e))?;

    let config = Config {
        recursive: args.recursive,
        ignore_hidden: args.ignore_hidden,
        include: args.include.clone(),
        exclude: args.exclude.clone(),
    };

    // --detail not supported yet
    if args.detail {
        eprintln!(
            "{}",
            style("Warning: --detail feature is not yet implemented.").yellow()
        );
    }

    if args.json {
        run_json(&args, &config)
    } else {
        run_human(&args, &config)
    }
}

fn run_json(args: &Args, config: &Config) -> Result<()> {
    let counts = count_extensions(&args.path, config)?;
    // Print original data
    let json_output = serde_json::to_string_pretty(&counts)?;
    println!("{}", json_output);
    Ok(())
}

fn run_human(args: &Args, config: &Config) -> Result<()> {
    let pb = if args.quiet {
        ProgressBar::hidden()
    } else {
        let pb = ProgressBar::new_spinner();
        pb.enable_steady_tick(Duration::from_millis(120));
        pb.set_style(ProgressStyle::default_spinner().template("{spinner:.blue} Scanning...")?);
        pb
    };

    let counts = count_extensions(&args.path, config)?;
    let total_files: u32 = counts.values().sum();
    pb.finish_with_message(
        style(format!("Scan complete! Found {} files.", total_files))
            .green()
            .to_string(),
    );

    let mut entries: Vec<_> = counts.into_iter().collect();
    if entries.is_empty() {
        println!("\n{}", style("No matching files found.").yellow());
        return Ok(());
    }

    match args.sort_by {
        Some(SortKey::Name) => entries.sort_by(|a, b| a.0.cmp(&b.0)),
        Some(SortKey::Count) => entries.sort_by(|a, b| b.1.cmp(&a.1)),
        None => entries.sort_by(|a, b| a.0.cmp(&b.0)),
    }
    if args.reverse {
        entries.reverse();
    }

    let final_entries = if let Some(n) = args.top {
        entries.into_iter().take(n).collect()
    } else {
        entries
    };

    println!(
        "\nFile extension counts for: {}",
        style(args.path.to_string_lossy()).bold()
    );

    // Formatting output
    let max_ext_len = final_entries.iter().map(|(e, _)| e.len()).max().unwrap_or(0);
    let width = (max_ext_len + 1).max(10);

    for (ext, count) in final_entries {
        let ext_styled: StyledObject<String> = style(format!(".{}", ext)).cyan();
        let count_styled: StyledObject<u32> = style(count).yellow();
        println!("{:<width$} : {}", ext_styled, count_styled, width = width);
    }

    Ok(())
}
