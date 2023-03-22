use std::sync::{Arc, Mutex};
use std::thread::{self, JoinHandle};
use std::time::Duration;

use crate::buffer::Buffer;
use crate::config::Config;

// String for now
// Need read-locked reference to data
pub fn start_autosave_thread(buffer: Arc<Mutex<Buffer>>, config: &Config) -> JoinHandle<()> {
    let autosave_interval = config.autosave_interval.clone();
    thread::spawn(move || loop {
        // Write buffer to file

        // Sleep depending on config
        // There are other functions I can use in case I want to support decimal precision
        thread::sleep(Duration::from_secs(autosave_interval.into()));
    })
}
