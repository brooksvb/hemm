use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};
use std::thread::{self, JoinHandle};

use crossterm::event::{Event, KeyCode};

use crate::buffer::Buffer;
use crate::config::Config;

/// The user input is handled on its own thread in order to prevent the possibility
/// of an input event being missed between loops.

/// Starts thread that captures terminal input events
/// Currently no sleep between loops
///
/// This thread is responsible for giving the user a way to end
/// the program, otherwise, it won't end otherwise. The terminal being in raw mode prevents the
/// terminal from sending a SIGINT event because the ctrl+c keystroke will be captured
/// Maybe there is a way to propogate the event to the terminal to let it handle the signaling?
pub fn start_input_thread(
    buffer_handle: Arc<Mutex<Buffer>>,
    running_handle: Arc<AtomicBool>,
    _config: &Config,
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
                                buffer.textarea.insert_newline();
                            }
                            KeyCode::Esc => {
                                // Exit the program
                                running_handle.store(false, Ordering::SeqCst);
                                // TODO: Display message for user
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
