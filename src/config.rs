use std::path::PathBuf;

#[derive(Debug)]
pub struct Config {
    /// Using hemingway mode disables backspace and nav
    writing_mode: WritingMode,

    // requestFullscreen: bool, // Ask for fullscreen when window opens
    /// Output name for file. Defaults to generated pattern
    output_name: Option<String>,

    /// Pattern to generate output name
    /// User may configure the default
    output_pattern: String,

    /// Directory to place output file
    output_dir: PathBuf,
}
// maybe instead of defining "default" fields, first set them to config, then overwrite them with
// cli options

impl Config {
    /// Create Config from arguments and user config file
    pub fn new(cli: &crate::cli::Cli) -> Config {
        Config {
            writing_mode: match cli.hemingway.unwrap_or(false) {
                true => WritingMode::Hemingway,
                false => WritingMode::Regular,
            },
            output_name: Some(String::from("test.md")),
            output_pattern: String::from("{date}.md"),
            output_dir: PathBuf::from("./"),
        }
    }
}

#[derive(Debug)]
pub enum WritingMode {
    Regular,
    Hemingway, // Disable backspace and navigation
}
