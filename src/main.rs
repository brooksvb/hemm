use std::error::Error;
use std::io;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};
use std::thread;
use std::thread::JoinHandle;
use std::time::Duration;

use clap::Parser;
use crossterm::event::{DisableMouseCapture, EnableMouseCapture};
use crossterm::terminal::{
    disable_raw_mode, enable_raw_mode, is_raw_mode_enabled, EnterAlternateScreen,
    LeaveAlternateScreen,
};
use hemm::autosave::start_autosave_thread;
use hemm::buffer::Buffer;
use hemm::cli::Cli;
use hemm::config::Config;
use hemm::input::start_input_thread;
use hemm::timer::start_timer_thread;
use tui::{backend::CrosstermBackend, layout::Rect, Terminal};

fn main() -> Result<(), Box<dyn Error>> {
    let cli = Cli::parse();
    dbg!(&cli);

    // Get Config
    let config = Config::new(&cli);
    dbg!(&config);

    run(&config)?;

    Ok(())
}

fn run(config: &Config) -> Result<(), Box<dyn Error>> {
    // Shared variables
    let buffer = Arc::new(Mutex::new(Buffer::new(config).unwrap()));
    // When this becomes false, all threads and program should exit
    let running = Arc::new(AtomicBool::new(true));
    let elapsed_time = Arc::new(Mutex::new(Duration::default()));

    let r = Arc::clone(&running);
    // Set up SIGINT handler
    // FIXME: Seems like raw mode prevents SIGINT signal from generating
    ctrlc::set_handler(move || {
        r.store(false, Ordering::SeqCst);
    })
    .expect("Error setting Ctrl-C handler");

    // Prepare interface
    let mut stdout = io::stdout();
    if !is_raw_mode_enabled()? {
        enable_raw_mode()?;
        crossterm::execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    }
    let backend = CrosstermBackend::new(stdout);
    let mut term = Terminal::new(backend)?;

    // Start background backup thread
    let backup_thread: Option<JoinHandle<()>>;
    if config.use_autosave {
        backup_thread = Some(start_autosave_thread(
            Arc::clone(&buffer),
            Arc::clone(&running),
            &config,
        ));
    } else {
        backup_thread = None;
    }

    // Start timer
    let timer_thread: Option<JoinHandle<()>>;
    if config.show_timer {
        timer_thread = Some(start_timer_thread(
            Arc::clone(&elapsed_time),
            Arc::clone(&running),
            &config,
        ));
    } else {
        timer_thread = None;
    }

    // Start input thread
    let input_thread = start_input_thread(Arc::clone(&buffer), Arc::clone(&running), &config);

    // Main render loop
    while running.load(Ordering::SeqCst) {
        term.draw(|f| {
            let buffer = buffer.lock().unwrap();
            let buffer_widget = buffer.textarea.widget();
            let rectangle = Rect::new(0, 0, f.size().width, f.size().height);
            f.render_widget(buffer_widget, rectangle);
        })
        .unwrap();

        // TUI refresh rate
        thread::sleep(Duration::from_millis(50));
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
    let mut buffer = buffer.lock().unwrap();
    dbg!(&buffer);
    buffer.save()?;

    // Cleanup bak files
    // TODO

    // Terminal cleanup
    term.show_cursor().unwrap();
    disable_raw_mode().unwrap();
    crossterm::execute!(
        term.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )
    .unwrap();

    dbg!(&buffer);
    Ok(())
}
