use std::path::PathBuf;

use crate::cli::Cli;

#[derive(Debug)]
pub struct Config {
    /// Using hemingway mode disables backspace and nav
    pub writing_mode: WritingMode,

    /// Output name for file
    /// If None, output path will be generated from pattern
    output_name: PathBuf,

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

#[derive(PartialEq, Debug)]
pub enum WritingMode {
    Regular,
    Hemingway, // Disable backspace and navigation
}

impl Default for Config {
    fn default() -> Self {
        Self {
            writing_mode: WritingMode::Regular,
            output_name: "output.txt".into(),
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
        let default_config_path = PathBuf::from("$XDG_CONFIG_HOME/hemm/hemm.conf");
        let config_path = if let Some(config) = cli.config.clone() {
            config
        } else {
            default_config_path
        };

        // Second, any command-line arguments override previously found values.
        // TODO: Last, any config values that were not found in either, will be set to defaults
        Config {
            writing_mode: if let Some(writing_mode) = cli.hemingway {
                match writing_mode {
                    true => WritingMode::Hemingway,
                    false => WritingMode::Regular,
                }
            } else {
                default.writing_mode
            },
            output_name: cli.path.clone(),
            output_dir: cli.directory.clone().unwrap_or(default.output_dir),
            use_autosave: cli.autosave.unwrap_or(default.use_autosave),
            autosave_interval: cli.autosave_interval.unwrap_or(default.autosave_interval),
            show_timer: cli.timer.unwrap_or(default.show_timer),
            use_hard_indent: cli.use_hard_indent.unwrap_or(default.use_hard_indent),
            ..default
        }
    }

    /// Return output path based on config
    pub fn get_output_path(&self) -> PathBuf {
        // If absolute file path is given, it doesn't matter what directory is set to
        if self.output_name.is_absolute() {
            return self.output_name;
        }
        // Merge directory with output name
        let mut path = self.output_dir.clone();
        path.push(self.output_name);
        return path;
    }

    /// Return path of backup file
    pub fn get_bak_path(&self) -> PathBuf {
        let output_path = self.get_output_path();
        let parent_dir = output_path.parent().unwrap();
        let file_name = output_path.file_name().unwrap();
        // to_string_lossy will drop invalid characters
        let bak_file_name = file_name.to_string_lossy().to_string() + ".bak";
        parent_dir.join(bak_file_name)
    }
}

fn resolve_pattern(pattern: &String) -> Result<String, ConfigError> {
    // TODO: Resolve pattern
    Ok("out.txt".into())
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct ConfigError {
    message: String,
}

impl std::fmt::Display for ConfigError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl std::error::Error for ConfigError {}
