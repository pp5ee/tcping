use clap::{Arg, ArgAction, Command, Parser};

/// Command line interface configuration
#[derive(Parser, Debug, Clone)]
#[command(name = "tcping", version = "2.7.1", about = "TCP ping utility for measuring network connectivity and latency")]
pub struct Cli {
    /// Target hostname or IP address
    #[arg(required = true)]
    pub host: String,

    /// Target port number
    #[arg(required = true)]
    pub port: u16,

    /// Timeout in seconds for each probe
    #[arg(short = 't', long = "timeout", default_value = "5")]
    pub timeout: f64,

    /// Interval in seconds between probes
    #[arg(short = 'i', long = "interval", default_value = "1")]
    pub interval: f64,

    /// Number of probes to send (0 for infinite)
    #[arg(short = 'c', long = "count")]
    pub count: Option<u32>,

    /// Use IPv4 only
    #[arg(short = '4', long = "ipv4")]
    pub ipv4_only: bool,

    /// Use IPv6 only
    #[arg(short = '6', long = "ipv6")]
    pub ipv6_only: bool,

    /// Source interface or IP address
    #[arg(short = 'I', long = "interface")]
    pub interface: Option<String>,

    /// Disable color output
    #[arg(long = "no-color")]
    pub no_color: bool,

    /// Show timestamps
    #[arg(short = 'D', long = "timestamps")]
    pub timestamps: bool,

    /// Output in JSON format
    #[arg(long = "json")]
    pub json: bool,

    /// Pretty JSON formatting
    #[arg(long = "pretty")]
    pub pretty: bool,

    /// Output to CSV file
    #[arg(long = "csv")]
    pub csv: Option<String>,

    /// Output to SQLite database
    #[arg(long = "db")]
    pub db: Option<String>,

    /// Show source address
    #[arg(long = "show-source-address")]
    pub show_source_address: bool,

    /// Only show failed probes
    #[arg(long = "show-failures-only")]
    pub show_failures_only: bool,

    /// Retry hostname resolution after N failed probes
    #[arg(long = "retry-resolution", default_value = "0")]
    pub retry_resolution: u32,

    /// Verbosity level (0-3)
    #[arg(short = 'v', long = "verbose", action = ArgAction::Count)]
    pub verbose: u8,

    /// Check for updates
    #[arg(short = 'u', long = "check-updates")]
    pub check_updates: bool,
}

impl Cli {
    /// Validate CLI arguments
    pub fn validate(&self) -> Result<(), String> {
        // Validate timeout
        if self.timeout <= 0.0 {
            return Err("Timeout must be greater than 0".to_string());
        }

        // Validate interval
        if self.interval <= 0.0 {
            return Err("Interval must be greater than 0".to_string());
        }

        // Validate count
        if let Some(count) = self.count {
            if count == 0 {
                return Err("Count must be greater than 0".to_string());
            }
        }

        // Validate protocol family exclusivity
        if self.ipv4_only && self.ipv6_only {
            return Err("Cannot specify both --ipv4 and --ipv6".to_string());
        }

        Ok(())
    }
}