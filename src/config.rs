use crate::cli::Cli;
use std::time::Duration;

/// Runtime configuration for TCP ping
#[derive(Debug, Clone)]
pub struct Config {
    /// Target hostname or IP address
    pub hostname: String,

    /// Target port number
    pub port: u16,

    /// Protocol family preference
    pub protocol_family: ProtocolFamily,

    /// Retry hostname resolution after N failed probes
    pub retry_resolution: u32,

    /// Maximum number of probes (None for infinite)
    pub max_probes: Option<u32>,

    /// Interval between probes
    pub interval: Duration,

    /// Connection timeout
    pub timeout: Duration,

    /// Network interface to bind to
    pub interface: Option<String>,

    /// Output configuration
    pub output: OutputConfig,

    /// Verbosity level
    pub verbosity: u8,
}

/// Protocol family preference
#[derive(Debug, Clone, PartialEq)]
pub enum ProtocolFamily {
    /// Use IPv4 only
    IPv4Only,
    /// Use IPv6 only
    IPv6Only,
    /// Use either IPv4 or IPv6 (system default)
    Any,
}

/// Output configuration
#[derive(Debug, Clone)]
pub struct OutputConfig {
    /// JSON output format
    pub json: bool,

    /// Pretty JSON formatting
    pub pretty: bool,

    /// Colored output
    pub color: bool,

    /// Show timestamps
    pub timestamps: bool,

    /// CSV output file
    pub csv_file: Option<String>,

    /// SQLite database file
    pub db_file: Option<String>,

    /// Show source address
    pub show_source_address: bool,

    /// Only show failed probes
    pub show_failures_only: bool,
}

impl Config {
    /// Create configuration from CLI arguments
    pub fn from_cli(cli: &Cli) -> Result<Self, String> {
        // Parse target to extract host and port
        let (hostname, port) = cli.parse_target()?;

        // Validate CLI arguments
        cli.validate()?;

        // Determine protocol family
        let protocol_family = if cli.ipv4_only {
            ProtocolFamily::IPv4Only
        } else if cli.ipv6_only {
            ProtocolFamily::IPv6Only
        } else {
            ProtocolFamily::Any
        };

        // Create output configuration
        let output = OutputConfig {
            json: cli.json,
            pretty: cli.pretty,
            color: !cli.no_color,
            timestamps: cli.timestamps,
            csv_file: cli.csv.clone(),
            db_file: cli.db.clone(),
            show_source_address: cli.show_source_address,
            show_failures_only: cli.show_failures_only,
        };

        Ok(Config {
            hostname,
            port,
            protocol_family,
            retry_resolution: cli.retry_resolution,
            max_probes: cli.count,
            interval: Duration::from_secs_f64(cli.interval),
            timeout: Duration::from_secs_f64(cli.timeout),
            interface: cli.interface.clone(),
            output,
            verbosity: cli.verbose,
        })
    }

    /// Get the hostname from the configuration
    pub fn hostname(&self) -> String {
        self.hostname.clone()
    }
}