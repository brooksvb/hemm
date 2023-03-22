use std::path::PathBuf;

use clap::Parser;

/// Struct representing options and arguments that user inputs to the program.
///
/// Powered by clap crate annotations
#[derive(Parser, Debug)]
pub struct Cli {
    /// Optional output file path
    pub path: Option<PathBuf>,

    /// Sets a custom config file
    #[arg(short, long, value_name = "FILE")]
    pub config: Option<PathBuf>,

    /// Use hemingway mode (no backspace, default: false)
    #[arg(long)]
    pub hemingway: Option<bool>,

    /// Pattern to use when generating output name
    #[arg(short, long)]
    pub output_pattern: Option<String>,

    /// Output directory for file if full output path not given
    /// default: ./
    #[arg(short, long)]
    pub directory: Option<PathBuf>,

    /// Enable autosave in background
    /// default: true
    #[arg(short, long)]
    pub autosave: Option<bool>,

    /// Interval of autosave, in seconds
    #[arg(long = "interval", value_name = "INTERVAL")]
    pub autosave_interval: Option<u32>,

    /// Enable timer display
    #[arg(short, long)]
    pub timer: Option<bool>,

    /// Use '\t' for tab keypress
    #[arg(short, long)]
    pub use_hard_indent: Option<bool>,
}
