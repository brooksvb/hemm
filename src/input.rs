use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};
use std::thread::{self, JoinHandle};

use crossterm::event::{Event, KeyCode};

use crate::buffer::Buffer;
use crate::config::Config;

// String for now
// Need a write-lock on buffer
// Need reference to input, buffer, stop_handle, config
pub fn start_input_thread(
    buffer_handle: Arc<Mutex<Buffer>>,
    running_handle: Arc<AtomicBool>,
    config: &Config,
) -> JoinHandle<()> {
    thread::spawn(move || {
        while running_handle.load(Ordering::SeqCst) {
            if let Ok(evt) = crossterm::event::read() {
                match evt {
                    Event::Key(key_event) => {
                        let mut buffer = buffer_handle.lock().unwrap();
                        match key_event.code {
                            // TODO: handle navigation
                            // TODO: Implement hemingway mode check
                            KeyCode::Char(c) => {
                                buffer.textarea.insert_char(c);
                            }
                            KeyCode::Backspace => {
                                buffer.textarea.delete_char();
                            }
                            KeyCode::Enter => {
                                buffer.textarea.insert_char('\n');
                            }
                            KeyCode::Esc => {
                                // Exit the program
                                running_handle.store(false, Ordering::SeqCst);
                            }
                            _ => {}
                        }
                    }
                    _ => {}
                }
            }
        }
    })
}
