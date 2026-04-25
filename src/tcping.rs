use crate::config::{Config, ProtocolFamily};
use std::net::{SocketAddr, ToSocketAddrs};
use std::time::{Duration, Instant};
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

    /// Current streak of consecutive successes/failures
    pub current_streak: u32,

    /// Whether the current streak is for successes (true) or failures (false)
    pub current_streak_success: bool,

    /// Longest streak of consecutive successes
    pub longest_success_streak: u32,

    /// Longest streak of consecutive failures
    pub longest_failure_streak: u32,
}

/// Main TCPing implementation
pub struct TCPing {
    /// Configuration for the TCPing session
    config: Config,

    /// Statistics for the current session
    stats: Statistics,
}

impl TCPing {
    /// Create a new TCPing instance with the given configuration
    pub fn new(config: Config) -> Self {
        Self {
            config,
            stats: Statistics::default(),
        }
    }

    /// Run the TCPing session
    pub async fn run(&mut self) -> Result<(), String> {
        // Resolve target address
        let target_addrs = self.resolve_target().await?;

        if target_addrs.is_empty() {
            return Err(format!("Could not resolve target: {}:{}", self.config.hostname, self.config.port));
        }

        println!("TCPing {}:{} ({}) with 32 bytes of data:",
                 self.config.hostname,
                 self.config.port,
                 target_addrs[0]);

        // Main ping loop
        let mut probe_count = 0;

        loop {
            // Check if we've reached the probe limit
            if let Some(max_probes) = self.config.max_probes {
                if probe_count >= max_probes {
                    break;
                }
            }

            // Perform the probe
            let result = self.probe_target(&target_addrs[0]).await;

            // Update statistics
            self.update_stats(&result);

            // Output the result
            self.output_result(&result).await?;

            probe_count += 1;

            // Wait for the next probe
            if probe_count < self.config.max_probes.unwrap_or(u32::MAX) {
                tokio::time::sleep(self.config.interval).await;
            }
        }

        // Output final statistics
        self.output_final_stats().await?;

        Ok(())
    }

    /// Resolve the target address
    async fn resolve_target(&self) -> Result<Vec<SocketAddr>, String> {
        let target = &format!("{}:{}", self.config.hostname, self.config.port);

        // Handle host:port format
        let (host, port) = if let Some(colon_pos) = target.rfind(':') {
            let (host_part, port_part) = target.split_at(colon_pos);
            let port_str = &port_part[1..]; // Remove the ':'

            match port_str.parse::<u16>() {
                Ok(port) => (host_part, port),
                Err(_) => return Err(format!("Invalid port number: {}", port_str)),
            }
        } else {
            return Err(format!("Target must be in format 'host:port' or 'host port', got: {}", target));
        };

        // Resolve host to socket addresses
        let addrs = format!("{}:{}", host, port)
            .to_socket_addrs()
            .map_err(|e| format!("Failed to resolve {}: {}", host, e))?
            .collect::<Vec<_>>();

        if addrs.is_empty() {
            return Err(format!("No addresses found for {}", host));
        }

        Ok(addrs)
    }

    /// Perform a single TCP ping probe
    async fn probe_target(&self, target_addr: &SocketAddr) -> ProbeResult {
        let start_time = Instant::now();

        // Attempt TCP connection with timeout
        let connect_result = timeout(self.config.timeout, TcpStream::connect(target_addr)).await;

        match connect_result {
            Ok(Ok(stream)) => {
                // Connection successful - measure RTT
                let rtt = start_time.elapsed().as_secs_f64() * 1000.0; // Convert to milliseconds

                // Get local address
                let source_addr = stream.local_addr().ok();

                // Close the connection
                drop(stream);

                ProbeResult {
                    success: true,
                    rtt: Some(rtt),
                    error: None,
                    source_addr,
                    timestamp: start_time,
                    target_addr: *target_addr,
                }
            }
            Ok(Err(e)) => {
                // Connection failed
                ProbeResult {
                    success: false,
                    rtt: None,
                    error: Some(format!("Connection failed: {}", e)),
                    source_addr: None,
                    timestamp: start_time,
                    target_addr: *target_addr,
                }
            }
            Err(_) => {
                // Timeout occurred
                ProbeResult {
                    success: false,
                    rtt: None,
                    error: Some(format!("Connection timeout after {:?}", self.config.timeout)),
                    source_addr: None,
                    timestamp: start_time,
                    target_addr: *target_addr,
                }
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
        if result.success {
            if let Some(rtt) = result.rtt {
                println!("Reply from {}: time={:.2}ms", result.target_addr, rtt);
            }
        } else {
            if let Some(error) = &result.error {
                println!("Connection to {} failed: {}", result.target_addr, error);
            } else {
                println!("Connection to {} failed", result.target_addr);
            }
        }

        Ok(())
    }

    /// Output final statistics
    async fn output_final_stats(&self) -> Result<(), String> {
        println!("\n--- {}:{} TCP ping statistics ---", self.config.hostname, self.config.port);
        println!("Probes sent: {}", self.stats.total_probes);
        println!("Successful: {}", self.stats.successful_probes);
        println!("Failed: {}", self.stats.failed_probes);
        println!("Packet loss: {:.1}%", self.stats.packet_loss);

        if let (Some(min), Some(max), Some(avg)) = (self.stats.min_rtt, self.stats.max_rtt, self.stats.avg_rtt) {
            println!("RTT: min={:.2}ms, max={:.2}ms, avg={:.2}ms", min, max, avg);
        }

        Ok(())
    }

    /// Get current statistics
    pub fn stats(&self) -> &Statistics {
        &self.stats
    }
}