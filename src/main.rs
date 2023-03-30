//! # hemm
//!
//! A text editor
//!
//! ## Usage
//! `hemm <output_filename>`
//! `hemm -h`

use std::error::Error;
use std::io;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Condvar, Mutex};
use std::thread::JoinHandle;
use std::time::Duration;

use clap::Parser;
use crossterm::cursor::SetCursorStyle;
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
use tui::backend::CrosstermBackend;
use tui::layout::{Constraint, Direction, Layout};
use tui::text::Span;
use tui::widgets::Paragraph;
use tui::Terminal;

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
    let condvar = Arc::new(Condvar::new());
    let elapsed_time = Arc::new(Mutex::new(Duration::default()));

    // Set up SIGINT handler
    {
        let r = Arc::clone(&running);
        let cond = Arc::clone(&condvar);
        // FIXME: Seems like raw mode prevents SIGINT signal from generating
        ctrlc::set_handler(move || {
            r.store(false, Ordering::SeqCst);
            cond.notify_all();
        })
        .expect("Error setting Ctrl-C handler");
    }

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
            Arc::clone(&condvar),
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
            Arc::clone(&condvar),
            &config,
        ));
    } else {
        timer_thread = None;
    }

    // Start input thread
    let input_thread = start_input_thread(
        Arc::clone(&buffer),
        Arc::clone(&running),
        Arc::clone(&condvar),
        &config,
    );

    // FIXME: Cursor style does not change
    crossterm::execute!(term.backend_mut(), SetCursorStyle::SteadyBar).unwrap();

    // TODO: Create layout
    let layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            [
                Constraint::Min(1),    // TextArea
                Constraint::Length(1), // Status line
            ]
            .as_ref(),
        );

    // Main render loop
    let mutex = Mutex::new(());
    while running.load(Ordering::SeqCst) {
        term.draw(|f| {
            let chunks = layout.split(f.size());
            let textarea_layout = Layout::default()
                .direction(Direction::Vertical)
                .constraints(
                    [
                        Constraint::Length(2), // Margin
                        Constraint::Min(1),    // Main area
                        Constraint::Length(1), // One less row margin on bottom due to status line
                    ]
                    .as_ref(),
                )
                .horizontal_margin(4);
            let textarea_chunk = textarea_layout.split(chunks[0])[1];

            let mut buffer = buffer.lock().unwrap();
            let buffer_widget = buffer.textarea.widget();
            f.render_widget(buffer_widget, textarea_chunk);

            let status_line_layout = Layout::default()
                .direction(Direction::Horizontal)
                .constraints([Constraint::Min(1), Constraint::Max(1 + 4 + 1)]);
            let status_chunks = status_line_layout.split(chunks[1]);
            let message = match buffer.get_message() {
                Some(message) => message,
                None => "",
            };
            f.render_widget(Paragraph::new(Span::raw(message)), status_chunks[0]);
            // TODO: Render timer
        })
        .unwrap();

        // TUI refresh rate
        let guard = mutex.lock().unwrap();
        _ = condvar.wait_timeout(guard, Duration::from_millis(50));
    }

    // Join threads. They should wake up and stop ASAP from shared condvar.notify_all() call
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

    // Final output for user
    println!("Saved file to {:?}", config.get_output_path());

    Ok(())
}
