use clap::{ArgAction, Parser, ValueHint};

/// A TCP ping tool for measuring network connectivity and latency
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Cli {
    /// Target hostname or IP address
    #[arg(value_hint = ValueHint::Hostname)]
    pub host: String,

    /// Target port number
    #[arg(value_hint = ValueHint::Other)]
    pub port: u16,

    /// Use IPv4 only
    #[arg(short = '4', long)]
    pub ipv4_only: bool,

    /// Use IPv6 only
    #[arg(short = '6', long)]
    pub ipv6_only: bool,

    /// Retry hostname resolution after N failed probes
    #[arg(short = 'r', long, default_value = "0")]
    pub retry_resolution: u32,

    /// Stop after N probes
    #[arg(short = 'c', long)]
    pub count: Option<u32>,

    /// Interval between probes in seconds
    #[arg(short = 'i', long, default_value = "1.0")]
    pub interval: f64,

    /// Timeout duration in seconds
    #[arg(short = 't', long, default_value = "1.0")]
    pub timeout: f64,

    /// Bind to specific network interface
    #[arg(short = 'I', long)]
    pub interface: Option<String>,

    /// Output in JSON format
    #[arg(short = 'j', long)]
    pub json: bool,

    /// Pretty JSON formatting
    #[arg(long)]
    pub pretty: bool,

    /// Disable colored output
    #[arg(long)]
    pub no_color: bool,

    /// Show timestamps
    #[arg(short = 'D', long)]
    pub timestamps: bool,

    /// Save output to CSV file
    #[arg(long, value_hint = ValueHint::FilePath)]
    pub csv: Option<String>,

    /// Save output to SQLite database
    #[arg(long, value_hint = ValueHint::FilePath)]
    pub db: Option<String>,

    /// Show source address
    #[arg(long)]
    pub show_source_address: bool,

    /// Only show failed probes
    #[arg(long)]
    pub show_failures_only: bool,

    /// Verbose output
    #[arg(short = 'v', long, action = ArgAction::Count)]
    pub verbose: u8,

    /// Check for updates
    #[arg(short = 'u', long)]
    pub check_updates: bool,

}

impl Cli {
    /// Validate CLI arguments and return error if invalid combinations are found
    pub fn validate(&self) -> Result<(), String> {
        if self.ipv4_only && self.ipv6_only {
            return Err("Cannot specify both --ipv4-only and --ipv6-only".to_string());
        }

        if self.interval <= 0.0 {
            return Err("Interval must be greater than 0".to_string());
        }

        if self.timeout <= 0.0 {
            return Err("Timeout must be greater than 0".to_string());
        }

        Ok(())
    }
}