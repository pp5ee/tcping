use clap::Parser;
use std::process;

mod cli;
mod config;
mod tcping;
mod output;

use crate::cli::Cli;
use crate::config::Config;
use crate::tcping::TcpPing;

#[tokio::main]
async fn main() {
    // Initialize logging
    env_logger::init();

    // Parse command-line arguments
    let cli = Cli::parse();

    // Create configuration from CLI
    let config = match Config::from_cli(&cli) {
        Ok(config) => config,
        Err(e) => {
            eprintln!("Error: {}", e);
            process::exit(1);
        }
    };

    // Create TCP ping instance
    let mut tcping = TcpPing::new(config);

    // Run the TCP ping
    if let Err(e) = tcping.run().await {
        eprintln!("Error: {}", e);
        process::exit(1);
    }
}