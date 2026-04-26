use crate::config::{Config, ProtocolFamily};
use std::net::{SocketAddr, ToSocketAddrs};
use std::time::{Duration, Instant};
use std::sync::Arc;
use tokio::sync::Mutex;
use chrono::{DateTime, Utc};
use tokio::net::TcpStream;

/// Hostname change tracking
#[derive(Debug, Clone)]
pub struct HostnameChange {
    pub addr: String,
    pub when: DateTime<Utc>,
}

/// Longest time tracking for uptime/downtime
#[derive(Debug, Clone, Default)]
pub struct LongestTime {
    pub start: Option<DateTime<Utc>>,
    pub end: Option<DateTime<Utc>>,
    pub duration: Duration,
}

/// RTT statistics
#[derive(Debug, Clone, Default)]
pub struct RttResult {
    pub min: f64,
    pub max: f64,
    pub average: f64,
    pub has_results: bool,
}

/// Comprehensive statistics tracking
#[derive(Debug, Clone)]
pub struct TcpPingStats {
    pub start_time: DateTime<Utc>,
    pub end_time: Option<DateTime<Utc>>,
    pub start_of_uptime: Option<DateTime<Utc>>,
    pub start_of_downtime: Option<DateTime<Utc>>,
    pub last_successful_probe: Option<DateTime<Utc>>,
    pub last_unsuccessful_probe: Option<DateTime<Utc>>,
    pub longest_uptime: LongestTime,
    pub longest_downtime: LongestTime,
    pub rtt_values: Vec<f64>,
    pub hostname_changes: Vec<HostnameChange>,
    pub ongoing_successful_probes: u32,
    pub ongoing_unsuccessful_probes: u32,
    pub total_downtime: Duration,
    pub total_uptime: Duration,
    pub total_successful_probes: u32,
    pub total_unsuccessful_probes: u32,
    pub retried_hostname_lookups: u32,
    pub rtt_results: RttResult,
    pub dest_was_down: bool,
    pub dest_is_ip: bool,
}

impl Default for TcpPingStats {
    fn default() -> Self {
        Self {
            start_time: Utc::now(),
            end_time: None,
            start_of_uptime: None,
            start_of_downtime: None,
            last_successful_probe: None,
            last_unsuccessful_probe: None,
            longest_uptime: LongestTime::default(),
            longest_downtime: LongestTime::default(),
            rtt_values: Vec::new(),
            hostname_changes: Vec::new(),
            ongoing_successful_probes: 0,
            ongoing_unsuccessful_probes: 0,
            total_downtime: Duration::default(),
            total_uptime: Duration::default(),
            total_successful_probes: 0,
            total_unsuccessful_probes: 0,
            retried_hostname_lookups: 0,
            rtt_results: RttResult::default(),
            dest_was_down: false,
            dest_is_ip: false,
        }
    }
}

pub struct TcpPing {
    config: Config,
    stats: Arc<Mutex<TcpPingStats>>,
}

impl TcpPing {
    pub fn new(config: Config) -> Self {
        Self {
            config,
            stats: Arc::new(Mutex::new(TcpPingStats::default())),
        }
    }

    pub async fn ping(&self, host: &str, port: u16) -> Result<PingResult, Box<dyn std::error::Error>> {
        let addr = self.resolve_host(host, port)?;
        let start = Instant::now();

        match tokio::time::timeout(self.config.timeout, TcpStream::connect(addr)).await {
            Ok(Ok(stream)) => {
                let duration = start.elapsed();
                let source_addr = stream.local_addr()?.to_string();
                drop(stream); // Close the connection immediately
                Ok(PingResult {
                    success: true,
                    rtt: duration,
                    error: None,
                    source_addr: Some(source_addr),
                })
            }
            Ok(Err(e)) => Ok(PingResult {
                success: false,
                rtt: start.elapsed(),
                error: Some(e.to_string()),
                source_addr: None,
            }),
            Err(_) => Ok(PingResult {
                success: false,
                rtt: self.config.timeout,
                error: Some("timeout".to_string()),
                source_addr: None,
            }),
        }
    }

    fn resolve_host(&self, host: &str, port: u16) -> Result<SocketAddr, Box<dyn std::error::Error>> {
        // Use Rust's built-in DNS resolution
        let socket_addr = format!("{}:{}", host, port)
            .to_socket_addrs()?
            .next()
            .ok_or_else(|| format!("Could not resolve host: {}", host))?;

        Ok(socket_addr)
    }

    /// Calculate min/avg/max RTT values
    pub fn calc_min_avg_max_rtt(&self, time_arr: &[f64]) -> RttResult {
        let mut result = RttResult::default();
        let arr_len = time_arr.len();

        if arr_len > 0 {
            result.min = time_arr[0];
            result.has_results = true;
        }

        let mut sum = 0.0;
        for i in 0..arr_len {
            sum += time_arr[i];

            if time_arr[i] > result.max {
                result.max = time_arr[i];
            }

            if time_arr[i] < result.min {
                result.min = time_arr[i];
            }
        }

        if arr_len > 0 {
            result.average = sum / arr_len as f64;
        }

        result
    }

    /// Calculate longest uptime
    pub fn calc_longest_uptime(&self, stats: &mut TcpPingStats, duration: Duration) {
        if stats.start_of_uptime.is_none() || duration == Duration::default() {
            return;
        }

        let longest_uptime = LongestTime {
            start: stats.start_of_uptime,
            end: Some(Utc::now()),
            duration,
        };

        // First time calling this function
        if stats.longest_uptime.end.is_none() {
            stats.longest_uptime = longest_uptime;
            return;
        }

        if longest_uptime.duration >= stats.longest_uptime.duration {
            stats.longest_uptime = longest_uptime;
        }
    }

    /// Calculate longest downtime
    pub fn calc_longest_downtime(&self, stats: &mut TcpPingStats, duration: Duration) {
        if stats.start_of_downtime.is_none() || duration == Duration::default() {
            return;
        }

        let longest_downtime = LongestTime {
            start: stats.start_of_downtime,
            end: Some(Utc::now()),
            duration,
        };

        // First time calling this function
        if stats.longest_downtime.end.is_none() {
            stats.longest_downtime = longest_downtime;
            return;
        }

        if longest_downtime.duration >= stats.longest_downtime.duration {
            stats.longest_downtime = longest_downtime;
        }
    }
}

#[derive(Debug, Clone)]
pub struct PingResult {
    pub success: bool,
    pub rtt: Duration,
    pub error: Option<String>,
    pub source_addr: Option<String>,
}

// Add missing functions that tests expect
pub fn parse_host_port(input: &str) -> Result<(String, u16), Box<dyn std::error::Error>> {
    if let Some(colon_pos) = input.find(':') {
        let host = input[..colon_pos].to_string();
        let port_str = &input[colon_pos + 1..];
        let port = port_str.parse::<u16>()
            .map_err(|_| format!("Invalid port number: {}", port_str))?;
        Ok((host, port))
    } else {
        Err("Missing port in host:port format".into())
    }
}

pub fn validate_args(host: &str, port: u16) -> Result<(), Box<dyn std::error::Error>> {
    if host.is_empty() {
        return Err("Host cannot be empty".into());
    }

    if port == 0 {
        return Err("Port cannot be zero".into());
    }

    Ok(())
}

// Add missing types that tests expect
#[derive(Debug, PartialEq, Clone)]
pub enum IpVersion {
    V4,
    V6,
    Auto,
}

impl std::str::FromStr for IpVersion {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "4" | "v4" | "ipv4" => Ok(IpVersion::V4),
            "6" | "v6" | "ipv6" => Ok(IpVersion::V6),
            "auto" => Ok(IpVersion::Auto),
            _ => Err(format!("Invalid IP version: {}", s)),
        }
    }
}

// Add missing utility functions
pub fn parse_duration(s: &str) -> Result<Duration, Box<dyn std::error::Error>> {
    if s.ends_with('s') {
        let seconds = s.trim_end_matches('s').parse::<u64>()?;
        Ok(Duration::from_secs(seconds))
    } else if s.ends_with("ms") {
        let millis = s.trim_end_matches("ms").parse::<u64>()?;
        Ok(Duration::from_millis(millis))
    } else {
        Err(format!("Invalid duration format: {}", s).into())
    }
}

pub fn validate_count(count: u32) -> Result<(), Box<dyn std::error::Error>> {
    if count == 0 {
        return Err("Count cannot be zero".into());
    }

    if count > 1000000 {
        return Err("Count cannot exceed 1,000,000".into());
    }

    Ok(())
}

pub fn parse_ip_version(s: &str) -> Result<IpVersion, Box<dyn std::error::Error>> {
    s.parse().map_err(|e: String| e.into())
}

pub fn choose_ip_version(s: &str) -> Result<IpVersion, Box<dyn std::error::Error>> {
    parse_ip_version(s)
}

pub fn parse_ip_family(s: &str) -> Result<ProtocolFamily, Box<dyn std::error::Error>> {
    match s.to_lowercase().as_str() {
        "4" | "v4" | "ipv4" => Ok(ProtocolFamily::IPv4Only),
        "6" | "v6" | "ipv6" => Ok(ProtocolFamily::IPv6Only),
        "auto" => Ok(ProtocolFamily::Any),
        _ => Err(format!("Invalid IP family: {}", s).into()),
    }
}

pub fn choose_ip_family(s: &str) -> Result<ProtocolFamily, Box<dyn std::error::Error>> {
    parse_ip_family(s)
}

pub fn supports_color() -> bool {
    atty::is(atty::Stream::Stdout)
}

pub fn generate_timestamp() -> String {
    chrono::Local::now().format("%Y-%m-%d %H:%M:%S%.3f").to_string()
}

use crate::output::OutputManager;
use tokio::signal;
use tokio::time::interval;
use std::io;

impl TcpPing {
    pub async fn run(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let output_manager = OutputManager::new(&self.config.output);

        // Initialize statistics
        {
            let mut stats = self.stats.lock().await;
            stats.dest_is_ip = self.config.hostname.parse::<std::net::IpAddr>().is_ok();

            // Add initial hostname change
            stats.hostname_changes.push(HostnameChange {
                addr: self.config.hostname.clone(),
                when: Utc::now(),
            });
        }

        // Print start message
        output_manager.output_start(&self.config.hostname, self.config.port);

        // Create interval for pinging
        let mut ping_interval = interval(self.config.interval);

        // Set up signal handling for graceful shutdown
        let ctrl_c = signal::ctrl_c();
        tokio::pin!(ctrl_c);

        // Set up stdin monitoring for real-time stats
        let (stdin_tx, mut stdin_rx) = tokio::sync::mpsc::channel(1);
        if self.config.output.json {
            // For JSON output, don't monitor stdin
        } else {
            let stdin_tx_clone = stdin_tx.clone();
            tokio::spawn(async move {
                // Use a simpler approach that doesn't require holding a lock
                loop {
                    let mut buffer = String::new();
                    if io::stdin().read_line(&mut buffer).is_ok() {
                        if buffer.trim().is_empty() {
                            let _ = stdin_tx_clone.send(true).await;
                        }
                    }
                    tokio::time::sleep(Duration::from_millis(100)).await;
                }
            });
        }

        let mut probe_count = 0;

        loop {
            tokio::select! {
                _ = ping_interval.tick() => {
                    // Perform ping
                    let result = self.ping(&self.config.hostname, self.config.port).await?;

                    // Update statistics
                    self.update_stats(&result).await;

                    // Output the result
                    let stats = self.stats.lock().await;
                    output_manager.output_probe(&result, &*stats);

                    probe_count += 1;

                    // Check if we've reached the probe limit
                    if let Some(max_probes) = self.config.max_probes {
                        if probe_count >= max_probes {
                            break;
                        }
                    }
                }
                _ = &mut ctrl_c => {
                    // Handle Ctrl+C - graceful shutdown
                    break;
                }
                pressed_enter = stdin_rx.recv() => {
                    if pressed_enter.is_some() {
                        // Print statistics on Enter key
                        let stats = self.stats.lock().await;
                        output_manager.output_stats(&*stats);
                    }
                }
            }
        }

        // Print final statistics
        {
            let mut stats = self.stats.lock().await;
            stats.end_time = Some(Utc::now());
            output_manager.output_stats(&*stats);
        }

        Ok(())
    }

    /// Update statistics based on ping result
    async fn update_stats(&self, result: &PingResult) {
        let mut stats = self.stats.lock().await;
        let now = Utc::now();

        if result.success {
            // Handle successful probe
            stats.total_successful_probes += 1;
            stats.ongoing_successful_probes += 1;
            stats.ongoing_unsuccessful_probes = 0;
            stats.last_successful_probe = Some(now);

            // Add RTT value
            stats.rtt_values.push(result.rtt.as_secs_f64() * 1000.0); // Convert to milliseconds

            // Handle uptime/downtime tracking
            if stats.dest_was_down {
                stats.start_of_uptime = Some(now);
                if let Some(downtime_start) = stats.start_of_downtime {
                    let downtime = now.signed_duration_since(downtime_start).to_std().unwrap_or_default();
                    self.calc_longest_downtime(&mut stats, downtime);
                    stats.total_downtime += downtime;
                }
                stats.start_of_downtime = None;
                stats.dest_was_down = false;
            }

            if stats.start_of_uptime.is_none() {
                stats.start_of_uptime = Some(now);
            }

            // Calculate current uptime
            if let Some(uptime_start) = stats.start_of_uptime {
                let uptime = now.signed_duration_since(uptime_start).to_std().unwrap_or_default();
                stats.total_uptime += uptime;
            }
        } else {
            // Handle failed probe
            stats.total_unsuccessful_probes += 1;
            stats.ongoing_unsuccessful_probes += 1;
            stats.ongoing_successful_probes = 0;
            stats.last_unsuccessful_probe = Some(now);

            // Handle uptime/downtime tracking
            if !stats.dest_was_down {
                stats.start_of_downtime = Some(now);
                if let Some(uptime_start) = stats.start_of_uptime {
                    let uptime = now.signed_duration_since(uptime_start).to_std().unwrap_or_default();
                    self.calc_longest_uptime(&mut stats, uptime);
                    stats.total_uptime += uptime;
                }
                stats.start_of_uptime = None;
                stats.dest_was_down = true;
            }

            // Calculate current downtime
            if let Some(downtime_start) = stats.start_of_downtime {
                let downtime = now.signed_duration_since(downtime_start).to_std().unwrap_or_default();
                stats.total_downtime += downtime;
            }
        }

        // Update RTT results
        stats.rtt_results = self.calc_min_avg_max_rtt(&stats.rtt_values);
    }
}
