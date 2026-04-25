//! TCP Ping Library
//!
//! A Rust library for measuring TCP connectivity and latency.

pub mod cli;
pub mod config;
pub mod tcping;
pub mod output;

// Re-export main types for easy access
pub use cli::Cli;
pub use config::{Config, ProtocolFamily, OutputConfig};
pub use tcping::{TcpPing, ProbeResult, Statistics};
pub use output::{OutputManager, OutputFormatter, ConsoleFormatter, JsonFormatter};

/// Main entry point for programmatic usage
pub struct TcpPingBuilder {
    config: Config,
}

impl TcpPingBuilder {
    /// Create a new builder with default configuration
    pub fn new(host: String, port: u16) -> Self {
        use std::net::{IpAddr, SocketAddr};

        let target = SocketAddr::new(IpAddr::V4(std::net::Ipv4Addr::UNSPECIFIED), port);
        let config = Config {
            target,
            protocol_family: ProtocolFamily::Any,
            retry_resolution: 0,
            max_probes: None,
            interval: std::time::Duration::from_secs(1),
            timeout: std::time::Duration::from_secs(1),
            interface: None,
            output: OutputConfig {
                json: false,
                pretty: false,
                color: true,
                timestamps: false,
                csv_file: None,
                db_file: None,
                show_source_address: false,
                show_failures_only: false,
            },
            verbosity: 0,
        };

        Self { config }
    }

    /// Set the protocol family preference
    pub fn protocol_family(mut self, family: ProtocolFamily) -> Self {
        self.config.protocol_family = family;
        self
    }

    /// Set the maximum number of probes
    pub fn max_probes(mut self, count: Option<u32>) -> Self {
        self.config.max_probes = count;
        self
    }

    /// Set the interval between probes
    pub fn interval(mut self, interval: std::time::Duration) -> Self {
        self.config.interval = interval;
        self
    }

    /// Set the connection timeout
    pub fn timeout(mut self, timeout: std::time::Duration) -> Self {
        self.config.timeout = timeout;
        self
    }

    /// Build the TCP ping instance
    pub fn build(self) -> TcpPing {
        TcpPing::new(self.config)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_builder_creation() {
        let builder = TcpPingBuilder::new("example.com".to_string(), 80);
        let _tcping = builder.build();

        // This just tests that the builder can be created without panicking
        assert!(true);
    }
}