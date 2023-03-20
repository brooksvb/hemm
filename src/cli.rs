use std::path::PathBuf;

use clap::Parser;

///
#[derive(Parser, Debug)]
pub struct Cli {
    /// Optional output file path
    pub path: Option<PathBuf>,

    /// Sets a custom config file
    #[arg(short, long, value_name = "FILE")]
    pub config: Option<PathBuf>,

    /// Use hemingway mode (no backspace, default: FALSE)
    #[arg(long)]
    pub hemingway: Option<bool>,

    /// Pattern to use when generating output name
    #[arg(short, long)]
    pub output_pattern: Option<String>,
}
