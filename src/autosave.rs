use std::io::{stderr, Write};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Condvar, Mutex};
use std::thread::{self, JoinHandle};
use std::time::Duration;

use crate::buffer::Buffer;
use crate::config::Config;

/// Starts autosave thread.
/// Sleeps between loops by user-configurable amount.
pub fn start_autosave_thread(
    buffer: Arc<Mutex<Buffer>>,
    running_handle: Arc<AtomicBool>,
    condvar: Arc<Condvar>,
    config: &Config,
) -> JoinHandle<()> {
    let autosave_interval = config.autosave_interval.clone();
    thread::spawn(move || {
        let autosave_interval = Duration::from_secs(autosave_interval.into());
        let mutex = Mutex::new(());
        while running_handle.load(Ordering::SeqCst) {
            {
                // Write buffer to file
                let mut buffer = buffer.lock().unwrap();
                buffer.save().unwrap_or_else(|err| {
                    // TODO: notify about save error
                    buffer
                        .set_message(Some(String::from("Error when saving; saving to .bak file")));
                    // TODO: Set error marker or save message for user review after exiting program
                    // tell them that backup file was saved
                    stderr()
                        .write_all(
                            format!("Encountered error when saving file: {}", err).as_bytes(),
                        )
                        .unwrap_or(());
                    buffer.save_backup().unwrap_or_else(|err| {
                        stderr()
                            .write_all(
                                format!(
                                    "WARN: Encountered error when attemping to save backup: {}",
                                    err
                                )
                                .as_bytes(),
                            )
                            .unwrap_or(());
                    });
                });
            }

            let guard = mutex.lock().unwrap();
            let _ = condvar.wait_timeout(guard, autosave_interval).unwrap();
        }
    })
}
