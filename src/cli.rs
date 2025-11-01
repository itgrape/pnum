use clap::{Parser, ValueEnum};
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(
    name = "pnum",
    version = "0.1.0",
    about = "Elegantly count the number of files by extension in a directory."
)]
pub struct Args {
    /// Path to scan
    #[arg(default_value = ".")]
    pub path: PathBuf,

    /// Whether to sort the output
    #[arg(short = 's', long, value_enum)]
    pub sort_by: Option<SortKey>,

    /// Reverse the output order
    #[arg(short = 'r', long)]
    pub reverse: bool,

    /// Whether to scan subdirectories recursively
    #[arg(short = 'R', long)]
    pub recursive: bool,

    /// Whether to show statistics for each directory separately
    #[arg(short = 'd', long)]
    pub detail: bool,

    /// Whether to show results quietly
    #[arg(short = 'q', long)]
    pub quiet: bool,

    /// Whether to ignore hidden files and directories
    #[arg(long)]
    pub ignore_hidden: bool,

    /// Only include specified extensions (can be used multiple times, e.g., -i rs -i toml)
    #[arg(short = 'i', long)]
    pub include: Vec<String>,

    /// Exclude specified extensions (can be used multiple times, e.g., -x log -x tmp)
    #[arg(short = 'x', long)]
    pub exclude: Vec<String>,

    /// Only show the top N extensions
    #[arg(short = 'n', long)]
    pub top: Option<usize>,

    /// Whether to output results in JSON format
    #[arg(long)]
    pub json: bool,

    /// Manually specify the number of threads to use for scanning
    #[arg(short = 't', long, default_value_t = 1)]
    pub thread_num: u16,
}

#[derive(ValueEnum, Clone, Debug)]
pub enum SortKey {
    Name,
    Count,
}
