use std::{
    fmt::Debug,
    fs,
    io::{self, Write},
    path::PathBuf,
};

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
    /// Path of output file
    path: PathBuf,
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
            .field("path", &self.path)
            .field("modified", &self.modified)
            .field("file_already_existed", &self.file_already_existed)
            .finish()
    }
}

impl Buffer {
    pub fn new(config: &Config) -> io::Result<Self> {
        // TODO: Generate path
        let path = config.output_name.clone();

        let file_already_existed = path.exists();
        let mut textarea = if let Ok(md) = path.metadata() {
            if md.is_file() {
                let contents = fs::read_to_string(path.clone())?;
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
        // textarea.set_line_number_style(Style::default().fg(Color::DarkGray));
        Ok(Self {
            textarea,
            path,
            modified: false,
            file_already_existed,
        })
    }

    /// If modified, write file to output path
    pub fn save(&mut self) -> io::Result<()> {
        // FIXME: modified is not updated
        // if !self.modified {
        //     return Ok(());
        // }
        // TODO: Write-Failsafe: If any error occurs, attempt to write to .bak file
        let mut f = fs::File::create(&self.path)?;
        for line in self.textarea.lines() {
            f.write_all(line.as_bytes())?;
            f.write_all(b"\n")?;
        }
        self.modified = false;
        Ok(())
    }
}
