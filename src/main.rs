use clap::Parser;
use std::process;
use tokio;

mod cli;
mod config;
mod output;
mod tcping;

use crate::cli::Cli;
use crate::config::Config;
use crate::tcping::TcpPing;

#[tokio::main]
async fn main() {
    // Parse command line arguments
    let cli = Cli::parse();

    // Convert CLI arguments to configuration
    let config = match Config::from_cli(&cli) {
        Ok(config) => config,
        Err(e) => {
            eprintln!("Configuration error: {}", e);
            process::exit(1);
        }
    };

    // Create TCP ping instance
    let mut tcping = TcpPing::new(config);

    // Run the TCP ping session
    if let Err(e) = tcping.run().await {
        eprintln!("Error running TCP ping: {}", e);
        process::exit(1);
    }
}