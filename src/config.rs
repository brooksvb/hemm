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

#[derive(Debug)]
pub enum WritingMode {
    Regular,
    Hemingway, // Disable backspace and navigation
}

impl Default for Config {
    fn default() -> Self {
        Self {
            writing_mode: WritingMode::Regular,
            output_name: "out.txt".into(),
            output_pattern: "{date}.txt".into(),
            output_dir: "./".into(),
            use_autosave: true,
            autosave_interval: 15,
            show_timer: false,
            use_hard_indent: true,
        }
    }
}

impl Config {
    /// Create Config from arguments and user config file
    pub fn new(cli: &Cli) -> Config {
        let default = Self::default();

        // First, a user config file is checked for config values.
        // TODO: Check for config file. Use confy
        // Use XDG_CONFIG_HOME
        let default_config_path = "$XDG_CONFIG_HOME/hemm/hemm.conf";
        // let configPath = cli.config.unwrap_or(PathBuf::from(value))

        // TODO: Generate output_name based on pattern, config, args
        let output_name = default.output_name;

        // Second, any command-line arguments override previously found values.
        // Last, any config values that were not found, will be set to defaults
        Config {
            writing_mode: if let Some(writing_mode) = cli.hemingway {
                match writing_mode {
                    true => WritingMode::Hemingway,
                    false => WritingMode::Regular,
                }
            } else {
                default.writing_mode
            },
            output_name,
            output_dir: cli.directory.clone().unwrap_or(default.output_dir),
            use_autosave: cli.autosave.unwrap_or(default.use_autosave),
            autosave_interval: cli.autosave_interval.unwrap_or(default.autosave_interval),
            show_timer: cli.timer.unwrap_or(default.show_timer),
            use_hard_indent: cli.use_hard_indent.unwrap_or(default.use_hard_indent),
            ..default
        }
    }
}
