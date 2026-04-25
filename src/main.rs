use clap::Parser;
use tcping::{Cli, Config, TcpPing};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Parse command line arguments
    let cli = Cli::parse();

    // Validate CLI arguments
    if let Err(e) = cli.validate() {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }

    // Create configuration from CLI
    let config = match Config::from_cli(&cli) {
        Ok(config) => config,
        Err(e) => {
            eprintln!("Configuration error: {}", e);
            std::process::exit(1);
        }
    };

    // Create TCP ping instance
    let mut tcping = TcpPing::new(config);

    // Run TCP ping session
    if let Err(e) = tcping.run().await {
        eprintln!("TCP ping error: {}", e);
        std::process::exit(1);
    }

    Ok(())
}