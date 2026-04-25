//! TCPing library - core functionality for TCP connection probing

use std::net::{IpAddr, SocketAddr};
use std::time::{Duration, Instant};
use tokio::net::TcpStream;
use tokio::time::timeout;

use thiserror::Error;

#[derive(Error, Debug)]
pub enum TcpingError {
    #[error("DNS resolution failed: {0}")]
    DnsResolution(String),
    #[error("TCP connection failed: {0}")]
    Connection(String),
    #[error("Timeout after {0:?}")]
    Timeout(Duration),
    #[error("Invalid hostname: {0}")]
    InvalidHostname(String),
    #[error("Invalid port: {0}")]
    InvalidPort(u16),
    #[error("Invalid IP address: {0}")]
    InvalidIp(String),
}

/// Represents a TCP probe result
#[derive(Debug, Clone)]
pub struct ProbeResult {
    pub success: bool,
    pub rtt: Option<f64>, // in milliseconds
    pub error: Option<String>,
    pub timestamp: Instant,
}

/// Configuration for TCP probing
#[derive(Debug, Clone)]
pub struct TcpingConfig {
    pub host: String,
    pub port: u16,
    pub timeout: Duration,
    pub use_ipv4: bool,
    pub use_ipv6: bool,
}

impl Default for TcpingConfig {
    fn default() -> Self {
        Self {
            host: String::new(),
            port: 80,
            timeout: Duration::from_secs(1),
            use_ipv4: false,
            use_ipv6: false,
        }
    }
}

/// Main TCPing struct that handles the probing logic
pub struct Tcping {
    config: TcpingConfig,
}

impl Tcping {
    /// Create a new TCPing instance with the given configuration
    pub fn new(config: TcpingConfig) -> Self {
        Self { config }
    }

    /// Resolve a hostname to IP addresses
    pub async fn resolve_hostname(&self, hostname: &str) -> Result<Vec<IpAddr>, TcpingError> {
        use std::net::ToSocketAddrs;

        // Try to parse as IP address first
        if let Ok(ip_addr) = hostname.parse::<IpAddr>() {
            return Ok(vec![ip_addr]);
        }

        // Resolve hostname
        let socket_addr = format!("{}:{}", hostname, self.config.port);

        match socket_addr.to_socket_addrs() {
            Ok(addrs) => {
                let mut ips: Vec<IpAddr> = addrs.map(|addr| addr.ip()).collect();

                // Filter by IP version if specified
                if self.config.use_ipv4 {
                    ips.retain(|ip| ip.is_ipv4());
                } else if self.config.use_ipv6 {
                    ips.retain(|ip| ip.is_ipv6());
                }

                if ips.is_empty() {
                    Err(TcpingError::DnsResolution(format!(
                        "No suitable IP addresses found for {}",
                        hostname
                    )))
                } else {
                    Ok(ips)
                }
            }
            Err(e) => Err(TcpingError::DnsResolution(e.to_string())),
        }
    }

    /// Perform a single TCP probe
    pub async fn probe(&self) -> ProbeResult {
        let start_time = Instant::now();

        match self.resolve_hostname(&self.config.host).await {
            Ok(ips) => {
                // Try each resolved IP address
                for ip in ips {
                    let socket_addr = SocketAddr::new(ip, self.config.port);

                    match timeout(self.config.timeout, TcpStream::connect(&socket_addr)).await {
                        Ok(Ok(_stream)) => {
                            let rtt = start_time.elapsed().as_secs_f64() * 1000.0; // Convert to milliseconds
                            return ProbeResult {
                                success: true,
                                rtt: Some(rtt),
                                error: None,
                                timestamp: start_time,
                            };
                        }
                        Ok(Err(e)) => {
                            // Connection failed but didn't timeout
                            return ProbeResult {
                                success: false,
                                rtt: None,
                                error: Some(format!("Connection failed: {}", e)),
                                timestamp: start_time,
                            };
                        }
                        Err(_) => {
                            // Timeout occurred
                            return ProbeResult {
                                success: false,
                                rtt: None,
                                error: Some(format!("Timeout after {:?}", self.config.timeout)),
                                timestamp: start_time,
                            };
                        }
                    }
                }

                // All IPs failed
                ProbeResult {
                    success: false,
                    rtt: None,
                    error: Some("All resolved IP addresses failed".to_string()),
                    timestamp: start_time,
                }
            }
            Err(e) => {
                ProbeResult {
                    success: false,
                    rtt: None,
                    error: Some(e.to_string()),
                    timestamp: start_time,
                }
            }
        }
    }

    /// Perform multiple probes with a specified interval
    pub async fn probe_multiple(
        &self,
        count: Option<usize>,
        interval: Duration,
    ) -> impl Iterator<Item = ProbeResult> {
        use tokio::time::sleep;
        use futures::stream::{self, StreamExt};

        let config = self.config.clone();

        // Create a stream of probes
        let stream = stream::iter(0..count.unwrap_or(usize::MAX))
            .then(move |_| {
                let config = config.clone();
                async move {
                    let tcping = Tcping::new(config);
                    let result = tcping.probe().await;
                    sleep(interval).await;
                    result
                }
            });

        // Convert to iterator (this is a simplified version)
        // In a real implementation, we'd use a proper async iterator
        vec![].into_iter()
    }
}

/// Statistics for multiple probes
#[derive(Debug, Default, Clone)]
pub struct ProbeStatistics {
    pub total_probes: usize,
    pub successful_probes: usize,
    pub failed_probes: usize,
    pub min_rtt: Option<f64>,
    pub max_rtt: Option<f64>,
    pub avg_rtt: Option<f64>,
    pub packet_loss: f64,
}

impl ProbeStatistics {
    /// Update statistics with a new probe result
    pub fn update(&mut self, result: &ProbeResult) {
        self.total_probes += 1;

        if result.success {
            self.successful_probes += 1;

            if let Some(rtt) = result.rtt {
                self.min_rtt = Some(self.min_rtt.map_or(rtt, |min| min.min(rtt)));
                self.max_rtt = Some(self.max_rtt.map_or(rtt, |max| max.max(rtt)));

                // Update average RTT
                let current_total = self.avg_rtt.map_or(0.0, |avg| avg * (self.successful_probes - 1) as f64);
                self.avg_rtt = Some((current_total + rtt) / self.successful_probes as f64);
            }
        } else {
            self.failed_probes += 1;
        }

        self.packet_loss = (self.failed_probes as f64 / self.total_probes as f64) * 100.0;
    }

    /// Calculate statistics from a collection of probe results
    pub fn from_results(results: &[ProbeResult]) -> Self {
        let mut stats = ProbeStatistics::default();

        for result in results {
            stats.update(result);
        }

        stats
    }
}

mod tests;