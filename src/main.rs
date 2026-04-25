use clap::Parser;
use std::process;
use tcping::cli::Cli;
use tcping::config::Config;
use tcping::TCPing;

#[tokio::main]
async fn main() {
    // Parse command line arguments
    let cli = Cli::parse();

    // Create configuration from CLI
    let config = match Config::from_cli(&cli) {
        Ok(config) => config,
        Err(err) => {
            eprintln!("Error: {}", err);
            process::exit(1);
        }
    };

    // Create and run TCPing
    let mut tcping = TCPing::new(config);

    if let Err(err) = tcping.run().await {
        eprintln!("Error: {}", err);
        process::exit(1);
    }
}