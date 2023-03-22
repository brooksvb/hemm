use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Condvar, Mutex};
use std::thread::{self, JoinHandle};
use std::time::{Duration, Instant};

use crate::config::Config;

pub fn start_timer_thread(
    elapsed_time_handle: Arc<Mutex<Duration>>,
    running_handle: Arc<AtomicBool>,
    condvar: Arc<Condvar>,
    _config: &Config,
) -> JoinHandle<()> {
    thread::spawn(move || {
        let start_time = Instant::now();
        let mutex = Mutex::new(());
        let timer_interval = Duration::from_millis(100);
        while running_handle.load(Ordering::SeqCst) {
            let current_elapsed = Instant::now().duration_since(start_time);
            {
                *elapsed_time_handle.lock().unwrap() = current_elapsed;
            }

            // Sleep for a short duration before updating the elapsed time again
            // Condvar allows instant wakeup on signal
            let guard = mutex.lock().unwrap();
            let _ = condvar.wait_timeout(guard, timer_interval).unwrap();
        }
    })
}
