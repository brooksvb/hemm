use dirs::config_dir;
use std::{fs::File, io::Read, path::PathBuf};
use thiserror::Error;

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

        // TODO: Verify output directory exists and is writable

        Config {
            writing_mode: if let Some(writing_mode) = cli.hemingway {
                match writing_mode {
                    true => WritingMode::Hemingway,
                    false => WritingMode::Regular,
                }
            } else {
                default.writing_mode
            },
            output_name: cli.path.clone().unwrap(),
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
            return self.output_name.clone();
        }
        // Merge directory with output name
        let mut path = self.output_dir.clone();
        path.push(self.output_name.clone());
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

#[derive(Error, Debug)]
pub enum ConfigErrorType {
    #[error("Invalid config path")]
    InvalidConfigPath,

    #[error("Failed to read config file")]
    FileReadError,

    #[error("Failed to parse config file")]
    DeserializationError,
}

#[derive(Debug)]
pub struct ConfigError {
    pub error_type: ConfigErrorType,
    pub path: PathBuf,
}

impl std::fmt::Display for ConfigError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}: {}", self.error_type, self.path.to_string_lossy())
    }
}

impl std::error::Error for ConfigError {}

/// Consume cli, check for user config files, merge them and return a Config
pub fn load_user_config(mut cli: Cli) -> Result<Config, ConfigError> {
    // Check config locations
    let config_dir = config_dir().expect("Failed to get configuration directory");
    let default_config_path = config_dir.join("hemm").join("hemm.conf");
    let config_path = cli.config.clone().unwrap_or(default_config_path);

    if !config_path.is_file() {
        // Error for invalid config path only if option was specified
        if let Some(ref config_path) = cli.config {
            return Err(ConfigError {
                error_type: ConfigErrorType::InvalidConfigPath,
                path: config_path.clone(),
            });
        }
        // Return config from cli options if no config file
        return Ok(Config::new(&cli));
    }

    let mut config_file = File::open(&config_path).map_err(|_| ConfigError {
        error_type: ConfigErrorType::FileReadError,
        path: config_path.clone(),
    })?;
    let mut config_str = String::new();
    config_file
        .read_to_string(&mut config_str)
        .map_err(|_| ConfigError {
            error_type: ConfigErrorType::FileReadError,
            path: config_path.clone(),
        })?;

    let config_cli: Cli = serde_yaml::from_str(&config_str).map_err(|_| ConfigError {
        error_type: ConfigErrorType::DeserializationError,
        path: config_path.clone(),
    })?;
    cli.merge(config_cli);

    Ok(Config::new(&cli))
}
