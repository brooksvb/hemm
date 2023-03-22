use std::path::PathBuf;

use crate::cli::Cli;

#[derive(Debug)]
pub struct Config {
    /// Using hemingway mode disables backspace and nav
    pub writing_mode: WritingMode,

    /// Output name for file. Defaults to generated pattern
    pub output_name: PathBuf,

    /// Pattern to generate output name
    /// User may configure the default
    pub output_pattern: String,

    /// Directory to place output file
    pub output_dir: PathBuf,

    /// Whether or not to autosave in background
    /// If error occurs during autosave, an attempt will be made to save to `<original_output_path>.bak`
    pub use_autosave: bool,

    /// Number of seconds between autosave backups
    pub autosave_interval: u32,

    /// Whether or not to show timer in editor
    pub show_timer: bool,

    /// Whether or not <TAB> keypress should enter spaces or '\t' character
    /// default: true
    pub use_hard_indent: bool,
}
// maybe instead of defining "default" fields, first set them to config, then overwrite them with
// cli options

#[derive(Debug)]
pub enum WritingMode {
    Regular,
    Hemingway, // Disable backspace and navigation
}

impl Config {
    /// Create Config from arguments and user config file
    pub fn new(cli: &Cli) -> Config {
        // TODO: Check for config file
        // Use XDG_CONFIG_HOME
        // let configPath = cli.config.unwrap_or(PathBuf::from(value))

        // TODO: Generate output_name based on pattern, config, args

        Config {
            writing_mode: match cli.hemingway.unwrap_or(false) {
                true => WritingMode::Hemingway,
                false => WritingMode::Regular,
            },
            output_name: "test.md".into(),
            output_pattern: String::from("{date}.md"),
            output_dir: "./".into(),
            use_autosave: true,
            autosave_interval: 15,
            show_timer: true,
            use_hard_indent: true,
        }
    }
}
