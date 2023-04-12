use std::path::PathBuf;

use clap::Parser;
use serde::{Deserialize, Serialize};

/// Struct representing options and arguments that user inputs to the program.
///
/// Powered by clap crate annotations
#[derive(Parser, Debug, Serialize, Deserialize)]
pub struct Cli {
    #[serde(skip_serializing)]
    #[arg(
        required = true,
        help = "Output file path. If the file exists, it will be opened to resume editing."
    )]
    pub path: Option<PathBuf>,

    /// Sets a custom config filepath
    #[serde(skip_serializing)]
    #[arg(short, long, value_name = "FILE")]
    pub config: Option<PathBuf>,

    /// Use hemingway mode (no backspace, default: false)
    #[arg(long)]
    pub hemingway: Option<bool>,

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

    // TODO: Custom clap value_parser for timer duration
    /// Enable timer display
    #[arg(short, long)]
    pub timer: Option<bool>,

    /// Use '\t' for tab keypress
    #[arg(short, long)]
    pub use_hard_indent: Option<bool>,
}

macro_rules! merge_fields {
    ($self_:ident, $other:expr, $($field:ident),+ $(,)?) => {
        $(
            $self_.$field = $other.$field.or_else(|| $self_.$field.take());
        )+
    };
}

impl Cli {
    /// Values from other Cli overwrite values from self
    pub fn merge(&mut self, other: Cli) {
        merge_fields!(
            self,
            other,
            path,
            config,
            hemingway,
            directory,
            autosave,
            autosave_interval,
            timer,
            use_hard_indent,
        );
    }
}
