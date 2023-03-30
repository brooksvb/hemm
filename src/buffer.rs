use std::{
    fmt::Debug,
    fs,
    io::{self, Write},
    path::PathBuf,
    time::{Duration, Instant},
};

use tui::{
    style::Style,
    widgets::{Block, Borders, Wrap},
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
    /// Path to file output
    path: PathBuf,
    /// Path to backup file
    back_path: PathBuf,
    /// Modified since last save
    // FIXME: modified is not updated
    modified: bool,
    /// Whether or not file existed at beginning of program start
    file_already_existed: bool,
    /// A temporary message for the user
    message: Option<String>,
    /// Instant of last message, to check if expired
    message_instant: Option<Instant>,
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
                let mut textarea = TextArea::from(contents.lines());
                // When resuming file, move cursor to end
                textarea.move_cursor(tui_textarea::CursorMove::Bottom);
                textarea.move_cursor(tui_textarea::CursorMove::End);
                textarea
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
        let block = Block::default().borders(Borders::ALL);
        textarea.set_block(block);
        Ok(Self {
            textarea,
            path: config.get_output_path(),
            back_path: config.get_bak_path(),
            modified: false,
            file_already_existed,
            message: None,
            message_instant: None,
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

    /// Get message if valid
    /// Side effect: Automatically clear message if invalid
    pub fn get_message(&mut self) -> Option<&String> {
        if !self.is_message_valid() {
            return None;
        }
        self.message.as_ref()
    }

    pub fn set_message(&mut self, message: Option<String>) {
        self.message = message;
        self.message_instant = Some(Instant::now());
    }

    /// Check if message is still valid, clear if not and return None
    fn is_message_valid(&mut self) -> bool {
        if self.message == None {
            return false;
        }
        // Message expires after this length
        static EXPIRE_DUR: Duration = Duration::from_secs(3);
        match self.message_instant {
            Some(instant) => {
                if instant.elapsed() >= EXPIRE_DUR {
                    self.clear_message();
                    false
                } else {
                    true
                }
            }
            None => false,
        }
    }

    fn clear_message(&mut self) {
        self.message = None;
        self.message_instant = None;
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
