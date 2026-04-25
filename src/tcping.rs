use crate::config::{Config, ProtocolFamily};
use std::net::{SocketAddr, ToSocketAddrs};
use std::time::Instant;
use tokio::net::TcpStream;
use tokio::time::timeout;

/// Result of a TCP ping probe
#[derive(Debug, Clone)]
pub struct ProbeResult {
    /// Whether the connection was successful
    pub success: bool,

    /// Round-trip time in milliseconds (None if failed)
    pub rtt: Option<f64>,

    /// Error message if connection failed
    pub error: Option<String>,

    /// Source address used for the connection
    pub source_addr: Option<SocketAddr>,

    /// Timestamp when the probe started
    pub timestamp: Instant,

    /// Target address that was actually used (after DNS resolution)
    pub target_addr: SocketAddr,
}

/// Statistics for TCP ping session
#[derive(Debug, Clone, Default)]
pub struct Statistics {
    /// Total number of probes sent
    pub total_probes: u32,

    /// Number of successful probes
    pub successful_probes: u32,

    /// Number of failed probes
    pub failed_probes: u32,

    /// Minimum RTT in milliseconds
    pub min_rtt: Option<f64>,

    /// Maximum RTT in milliseconds
    pub max_rtt: Option<f64>,

    /// Average RTT in milliseconds
    pub avg_rtt: Option<f64>,

    /// Total RTT sum for calculating average
    pub total_rtt: f64,

    /// Packet loss percentage
    pub packet_loss: f64,

    /// Start time of the session
    pub start_time: Option<Instant>,

    /// Longest successful streak
    pub longest_success_streak: u32,

    /// Longest failure streak
    pub longest_failure_streak: u32,

    /// Current success/failure streak
    pub current_streak: u32,

    /// Whether current streak is success (true) or failure (false)
    pub current_streak_success: bool,
}

/// Main TCP ping engine
pub struct TcpPing {
    /// Configuration
    config: Config,

    /// Current statistics
    stats: Statistics,

    /// Resolved target addresses
    resolved_targets: Vec<SocketAddr>,

    /// Current target index for round-robin
    current_target_index: usize,
}

impl TcpPing {
    /// Create a new TCP ping instance
    pub fn new(config: Config) -> Self {
        Self {
            config,
            stats: Statistics::default(),
            resolved_targets: Vec::new(),
            current_target_index: 0,
        }
    }

    /// Run the TCP ping session
    pub async fn run(&mut self) -> Result<(), String> {
        // Resolve target addresses
        self.resolve_targets().await?;

        if self.resolved_targets.is_empty() {
            return Err("No valid addresses found for target".to_string());
        }

        // Initialize statistics
        self.stats.start_time = Some(Instant::now());

        // Main ping loop
        let mut probe_count = 0;
        let mut consecutive_failures = 0;

        loop {
            // Check if we should stop
            if let Some(max_probes) = self.config.max_probes {
                if probe_count >= max_probes {
                    break;
                }
            }

            // Perform the probe
            let result = self.probe().await;

            // Update statistics
            self.update_stats(&result);

            // Update consecutive failures count for hostname retry logic
            if result.success {
                consecutive_failures = 0;
            } else {
                consecutive_failures += 1;
            }

            // Check if we need to retry hostname resolution
            if self.config.retry_resolution > 0 && consecutive_failures >= self.config.retry_resolution {
                println!("Retrying hostname resolution after {} consecutive failures", consecutive_failures);
                self.resolve_targets().await?;
                consecutive_failures = 0; // Reset after retry
            }

            // Output the result
            self.output_result(&result).await?;

            probe_count += 1;

            // Wait for the next interval or Ctrl+C
            tokio::select! {
                _ = tokio::time::sleep(self.config.interval) => {},
                _ = tokio::signal::ctrl_c() => {
                    println!("\nReceived interrupt signal, stopping...");
                    break;
                }
            }
        }

        // Output final statistics
        self.output_final_stats().await?;

        Ok(())
    }

    /// Resolve target addresses based on configuration
    async fn resolve_targets(&mut self) -> Result<(), String> {
        let hostname = self.config.hostname();
        let port = self.config.port;

        // Create socket address string for resolution
        let addr_string = format!("{}:{}", hostname, port);

        // Resolve addresses
        let addresses: Vec<SocketAddr> = addr_string
            .to_socket_addrs()
            .map_err(|e| format!("Failed to resolve {}: {}", addr_string, e))?
            .collect();

        if addresses.is_empty() {
            return Err(format!("No addresses found for {}", addr_string));
        }

        // Filter addresses based on protocol family preference
        self.resolved_targets = match self.config.protocol_family {
            ProtocolFamily::IPv4Only => addresses
                .into_iter()
                .filter(|addr| addr.is_ipv4())
                .collect(),
            ProtocolFamily::IPv6Only => addresses
                .into_iter()
                .filter(|addr| addr.is_ipv6())
                .collect(),
            ProtocolFamily::Any => addresses,
        };

        if self.resolved_targets.is_empty() {
            return Err(format!(
                "No {} addresses found for {}",
                match self.config.protocol_family {
                    ProtocolFamily::IPv4Only => "IPv4",
                    ProtocolFamily::IPv6Only => "IPv6",
                    ProtocolFamily::Any => "valid",
                },
                addr_string
            ));
        }

        Ok(())
    }

    /// Perform a single TCP ping probe
    async fn probe(&mut self) -> ProbeResult {
        let start_time = Instant::now();

        // Get next target address (round-robin)
        let target_addr = self.resolved_targets[self.current_target_index];
        self.current_target_index = (self.current_target_index + 1) % self.resolved_targets.len();

        // Attempt TCP connection with timeout and optional interface binding
        let connection_result = if let Some(interface) = &self.config.interface {
            // Use interface binding if specified
            self.connect_with_interface(&target_addr, interface).await
        } else {
            // Use standard connection
            timeout(self.config.timeout, TcpStream::connect(&target_addr)).await
        };

        match connection_result {
            Ok(Ok(stream)) => {
                let rtt = start_time.elapsed().as_secs_f64() * 1000.0; // Convert to milliseconds
                let source_addr = stream.local_addr().ok();

                // Close the connection immediately
                drop(stream);

                ProbeResult {
                    success: true,
                    rtt: Some(rtt),
                    error: None,
                    source_addr,
                    timestamp: start_time,
                    target_addr,
                }
            }
            Ok(Err(e)) => ProbeResult {
                success: false,
                rtt: None,
                error: Some(format!("Connection failed: {}", e)),
                source_addr: None,
                timestamp: start_time,
                target_addr,
            },
            Err(_) => ProbeResult {
                success: false,
                rtt: None,
                error: Some(format!("Connection timeout after {:?}", self.config.timeout)),
                source_addr: None,
                timestamp: start_time,
                target_addr,
            },
        }
    }

    /// Connect to target with specific interface binding
    async fn connect_with_interface(&self, target_addr: &SocketAddr, interface: &str) -> Result<Result<TcpStream, std::io::Error>, tokio::time::error::Elapsed> {
        use std::net::IpAddr;

        // Parse interface as IP address
        match interface.parse::<IpAddr>() {
            Ok(interface_ip) => {
                // Interface is an IP address - bind to it
                let local_addr = SocketAddr::new(interface_ip, 0); // Use port 0 for automatic assignment

                // Create a custom dialer with local address binding
                let dialer = match tokio::net::TcpSocket::new_v4()
                    .or_else(|_| tokio::net::TcpSocket::new_v6())
                {
                    Ok(dialer) => dialer,
                    Err(e) => return Ok(Err(std::io::Error::new(std::io::ErrorKind::Other, e))),
                };

                // Bind to the local address
                match dialer.bind(local_addr) {
                    Ok(_) => {},
                    Err(e) => return Ok(Err(std::io::Error::new(std::io::ErrorKind::AddrNotAvailable, e))),
                }

                // Connect with timeout
                timeout(self.config.timeout, dialer.connect(*target_addr)).await
            }
            Err(_) => {
                // Interface name parsing failed - treat as unsupported feature
                Ok(Err(std::io::Error::new(
                    std::io::ErrorKind::Unsupported,
                    format!("Interface binding by name '{}' requires platform-specific implementation. Use IP address instead.", interface)
                )))
            }
        }
    }

    /// Update statistics based on probe result
    fn update_stats(&mut self, result: &ProbeResult) {
        self.stats.total_probes += 1;

        if result.success {
            self.stats.successful_probes += 1;

            if let Some(rtt) = result.rtt {
                // Update min/max RTT
                self.stats.min_rtt = Some(self.stats.min_rtt.map_or(rtt, |min| min.min(rtt)));
                self.stats.max_rtt = Some(self.stats.max_rtt.map_or(rtt, |max| max.max(rtt)));

                // Update average RTT
                self.stats.total_rtt += rtt;
                self.stats.avg_rtt = Some(self.stats.total_rtt / self.stats.successful_probes as f64);
            }

            // Update streaks
            if self.stats.current_streak_success {
                self.stats.current_streak += 1;
            } else {
                self.stats.current_streak = 1;
                self.stats.current_streak_success = true;
            }
            self.stats.longest_success_streak = self.stats.longest_success_streak.max(self.stats.current_streak);
        } else {
            self.stats.failed_probes += 1;

            // Update streaks
            if !self.stats.current_streak_success {
                self.stats.current_streak += 1;
            } else {
                self.stats.current_streak = 1;
                self.stats.current_streak_success = false;
            }
            self.stats.longest_failure_streak = self.stats.longest_failure_streak.max(self.stats.current_streak);
        }

        // Update packet loss percentage
        self.stats.packet_loss = (self.stats.failed_probes as f64 / self.stats.total_probes as f64) * 100.0;
    }

    /// Output a single probe result
    async fn output_result(&self, result: &ProbeResult) -> Result<(), String> {
        let output_manager = crate::output::OutputManager::new(&self.config.output);
        output_manager.output_probe(result);
        Ok(())
    }

    /// Output final statistics
    async fn output_final_stats(&self) -> Result<(), String> {
        let output_manager = crate::output::OutputManager::new(&self.config.output);
        output_manager.output_stats(&self.stats);
        Ok(())
    }

    /// Get current statistics
    pub fn stats(&self) -> &Statistics {
        &self.stats
    }
}