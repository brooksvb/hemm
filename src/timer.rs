use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};
use std::thread::{self, JoinHandle};
use std::time::{Duration, Instant};

use crate::config::Config;

pub fn start_timer_thread(
    elapsed_time_handle: Arc<Mutex<Duration>>,
    running_handle: Arc<AtomicBool>,
    _config: &Config,
) -> JoinHandle<()> {
    thread::spawn(move || {
        let start_time = Instant::now();
        while running_handle.load(Ordering::SeqCst) {
            let current_elapsed = Instant::now().duration_since(start_time);
            *elapsed_time_handle.lock().unwrap() = current_elapsed;

            // Sleep for a short duration before updating the elapsed time again
            thread::sleep(Duration::from_millis(100));
        }
    })
}
