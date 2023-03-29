use std::{
    fmt::Debug,
    fs,
    io::{self, Write},
    path::PathBuf,
};

use tui::{style::Style, widgets::Wrap};
use tui_textarea::TextArea;

use crate::config::Config;

// Code largely adapted from tui-textarea editor example
// https://github.com/rhysd/tui-textarea/blob/d4bbccbfdbf8c8be933c30c1f7ee61be2f18b6b4/examples/editor.rs

/// A Buffer contains the text of the file being actively written
pub struct Buffer {
    // Using static lifetime to avoid errors when referencing in other threads
    // Not 100% understanding the full reasons yet or a better alternative solution
    /// TextArea tui widget
    pub textarea: TextArea<'static>,
    /// Path to file output
    path: PathBuf,
    /// Path to backup file
    back_path: PathBuf,
    /// Modified since last save
    // FIXME: modified is not updated
    modified: bool,
    /// Whether or not file existed at beginning of program start
    file_already_existed: bool,
}

impl Debug for Buffer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Buffer")
            .field("textarea", &self.textarea.lines())
            .field("modified", &self.modified)
            .field("file_already_existed", &self.file_already_existed)
            .finish()
    }
}

impl Buffer {
    pub fn new(config: &Config) -> io::Result<Self> {
        let path = config.get_output_path();

        let file_already_existed = path.exists();
        let mut textarea = if let Ok(md) = path.metadata() {
            if md.is_file() {
                let contents = fs::read_to_string(path.clone())?;
                // FIXME: When resuming file, move cursor to end
                TextArea::from(contents.lines())
            } else {
                // Path exists but is not a file
                return Err(io::Error::new(
                    io::ErrorKind::Other,
                    format!("{:?} exists but is not a file", path),
                ));
            }
        } else {
            TextArea::default() // File does not exist
        };
        textarea.set_hard_tab_indent(config.use_hard_indent);
        // Remove default underline style from active line
        textarea.set_cursor_line_style(Style::default());
        textarea.set_wrap(Some(Wrap { trim: false }));
        Ok(Self {
            textarea,
            path: config.get_output_path(),
            back_path: config.get_bak_path(),
            modified: false,
            file_already_existed,
        })
    }

    /// Save to backup filepath
    pub fn save_backup(&mut self) -> io::Result<()> {
        save_buffer(&self, &self.back_path)?;
        Ok(())
    }

    /// Save to final filepath
    pub fn save(&mut self) -> io::Result<()> {
        save_buffer(&self, &self.path)?;
        Ok(())
    }
}

fn save_buffer(buffer: &Buffer, path: &PathBuf) -> io::Result<()> {
    // FIXME: modified is not updated
    // if !self.modified {
    //     return Ok(());
    //
    // TODO: Write-Failsafe: If any error occurs, attempt to write to .bak file
    let mut f = fs::File::create(path)?;
    for line in buffer.textarea.lines() {
        f.write_all(line.as_bytes())?;
        f.write_all(b"\n")?;
    }
    Ok(())
}
