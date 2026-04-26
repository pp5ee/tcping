use clap::Parser;
use std::env;
use std::process;
use tcping::cli::Cli;
use tcping::config::Config;
use tcping::TcpPing;

#[tokio::main]
async fn main() {
    // Check for version flag manually before parsing CLI
    let args: Vec<String> = env::args().collect();
    if args.iter().any(|arg| arg == "-v" || arg == "--version") {
        println!("tcping 2.7.1");
        return;
    }

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
    let mut tcping = TcpPing::new(config);

    if let Err(err) = tcping.run().await {
        eprintln!("Error: {}", err);
        process::exit(1);
    }
}