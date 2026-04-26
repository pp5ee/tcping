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

    // Parse command line arguments with host:port support
    let cli = match parse_cli_with_host_port(&args) {
        Ok(cli) => cli,
        Err(err) => {
            eprintln!("Error: {}", err);
            process::exit(1);
        }
    };

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

/// Parse CLI with support for host:port format
fn parse_cli_with_host_port(args: &[String]) -> Result<Cli, String> {
    // Handle host:port format by splitting the last argument if needed
    let mut processed_args = args.to_vec();

    if args.len() >= 2 {
        let last_arg = &args[args.len() - 1];

        // Check for host:port format
        if last_arg.contains(':') && !last_arg.starts_with('-') {
            let parts: Vec<&str> = last_arg.splitn(2, ':').collect();
            if parts.len() == 2 {
                processed_args.pop(); // Remove the combined argument
                processed_args.push(parts[0].to_string());
                processed_args.push(parts[1].to_string());
            }
        }

        // Handle IPv6 addresses with brackets [::1]:8080
        if last_arg.starts_with('[') && last_arg.contains("]:") {
            if let Some(bracket_end) = last_arg.find(']') {
                if last_arg.len() > bracket_end + 1 && &last_arg[bracket_end..bracket_end+2] == "]:" {
                    let host = &last_arg[1..bracket_end];
                    let port = &last_arg[bracket_end+2..];
                    processed_args.pop(); // Remove the combined argument
                    processed_args.push(host.to_string());
                    processed_args.push(port.to_string());
                }
            }
        }
    }

    Cli::try_parse_from(processed_args).map_err(|e| e.to_string())
}