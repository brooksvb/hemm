use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Condvar, Mutex};
use std::thread::{self, JoinHandle};

use crossterm::event::{Event, KeyCode};
use tui_textarea::CursorMove;

use crate::buffer::Buffer;
use crate::config::{Config, WritingMode};

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
    condvar: Arc<Condvar>,
    config: &Config,
) -> JoinHandle<()> {
    let hemingway_mode = config.writing_mode == WritingMode::Hemingway;
    thread::spawn(move || {
        while running_handle.load(Ordering::SeqCst) {
            if let Ok(evt) = crossterm::event::read() {
                match evt {
                    Event::Key(key_event) => {
                        let mut buffer = buffer_handle.lock().unwrap();
                        match key_event.code {
                            KeyCode::Char(c) => {
                                buffer.textarea.insert_char(c);
                                buffer.mark_modified();
                            }
                            KeyCode::Left
                            | KeyCode::Right
                            | KeyCode::Up
                            | KeyCode::Down
                            | KeyCode::Home
                            | KeyCode::End
                                if !hemingway_mode =>
                            {
                                // PageUp PageDown not implemented
                                buffer.textarea.move_cursor(match key_event.code {
                                    KeyCode::Left => CursorMove::Back,
                                    KeyCode::Right => CursorMove::Forward,
                                    KeyCode::Up => CursorMove::Up,
                                    KeyCode::Down => CursorMove::Down,
                                    KeyCode::Home => CursorMove::Head,
                                    KeyCode::End => CursorMove::End,
                                    _ => unreachable!(),
                                });
                            }
                            KeyCode::PageUp if !hemingway_mode => {
                                buffer.textarea.scroll((-10, 0));
                            }
                            KeyCode::PageDown if !hemingway_mode => {
                                buffer.textarea.scroll((10, 0));
                            }
                            KeyCode::Backspace if !hemingway_mode => {
                                buffer.textarea.delete_char();
                                buffer.mark_modified();
                            }
                            KeyCode::Delete if !hemingway_mode => {
                                buffer.textarea.delete_next_char();
                                buffer.mark_modified();
                            }
                            KeyCode::Enter => {
                                buffer.textarea.insert_newline();
                                buffer.mark_modified();
                            }
                            KeyCode::Tab => {
                                buffer.textarea.insert_tab();
                                buffer.mark_modified();
                            }
                            KeyCode::Esc => {
                                // Exit the program
                                running_handle.store(false, Ordering::SeqCst);
                                // Wake up all sleeping threads
                                condvar.notify_all();
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
