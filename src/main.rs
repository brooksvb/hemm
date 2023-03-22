use std::io::{stdout, Write};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};
use std::thread::JoinHandle;
use std::time::Duration;

use clap::Parser;
use hemm::autosave::start_autosave_thread;
use hemm::buffer::Buffer;
use hemm::cli::Cli;
use hemm::config::Config;
use hemm::input::start_input_thread;
use hemm::timer::start_timer_thread;
use tui::{backend::CrosstermBackend, layout::Rect, Terminal};
use tui_textarea::TextArea;

fn main() {
    let cli = Cli::parse();
    dbg!(&cli);

    // Get Config
    let config = Config::new(&cli);
    dbg!(&config);

    run(&config);
}

fn run(config: &Config) {
    // Shared variables
    let buffer = Arc::new(Mutex::new(Buffer::new(config).unwrap()));
    let running = Arc::new(AtomicBool::new(true));
    let elapsed_time = Arc::new(Mutex::new(Duration::default()));

    // Prepare interface

    // Start background backup thread
    let backup_thread: Option<JoinHandle<()>>;
    if config.use_autosave {
        backup_thread = Some(start_autosave_thread(Arc::clone(&buffer), &config));
    } else {
        backup_thread = None;
    }

    // Start timer
    let timer_thread: Option<JoinHandle<()>>;
    if config.show_timer {
        timer_thread = Some(start_timer_thread(Arc::clone(&elapsed_time), &config));
    } else {
        timer_thread = None;
    }

    // Start input thread
    let input_thread = start_input_thread(Arc::clone(&buffer), Arc::clone(&running), &config);

    // Main render loop
    while running.load(Ordering::SeqCst) {
        // TODO
    }

    // TODO: Join threads
    if let Some(backup_thread) = backup_thread {
        backup_thread.join().unwrap();
    }
    if let Some(timer_thread) = timer_thread {
        timer_thread.join().unwrap();
    }
    input_thread.join().unwrap();

    // Final save
    // Cleanup bak files
}
