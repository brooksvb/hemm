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
                buffer.save().unwrap_or_else(|_| {
                    // TODO: notify about save error
                });
            }

            let guard = mutex.lock().unwrap();
            let _ = condvar.wait_timeout(guard, autosave_interval).unwrap();
        }
    })
}
