use clap::Parser;
use hemm::cli::Cli;
use hemm::config::Config;

fn main() {
    let cli = Cli::parse();

    dbg!(&cli);

    // TODO: Check for config file and parse

    // Generate Config
    let config = Config::new(&cli);

    dbg!(&config);

    run(&config);
}

fn run(config: &Config) {
    // Load config
    // Open file
    // Initialize buffer
    // Prepare interface
    // Start background backup thread
    // Start timer

    // Input loop
}
