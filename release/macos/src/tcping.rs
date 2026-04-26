use crate::config::{Config, ProtocolFamily};
use std::net::{IpAddr, SocketAddr, TcpStream, ToSocketAddrs};
use std::time::{Duration, Instant};

pub struct TcpPing {
    config: Config,
}

impl TcpPing {
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    pub fn ping(&self, host: &str, port: u16) -> Result<PingResult, Box<dyn std::error::Error>> {
        let addr = self.resolve_host(host, port)?;
        let start = Instant::now();

        match TcpStream::connect_timeout(&addr, self.config.timeout) {
            Ok(_) => {
                let duration = start.elapsed();
                Ok(PingResult {
                    success: true,
                    rtt: duration,
                    error: None,
                })
            }
            Err(e) => Ok(PingResult {
                success: false,
                rtt: start.elapsed(),
                error: Some(e.to_string()),
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
}

#[derive(Debug, Clone)]
pub struct PingResult {
    pub success: bool,
    pub rtt: Duration,
    pub error: Option<String>,
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

// Add Statistics struct that tests expect
#[derive(Debug, Default)]
pub struct Statistics {
    pub total: u32,
    pub successful: u32,
    pub failed: u32,
    pub min_rtt: Duration,
    pub max_rtt: Duration,
    pub total_rtt: Duration,
}

impl Statistics {
    pub fn new() -> Self {
        Self {
            min_rtt: Duration::from_secs(u64::MAX),
            max_rtt: Duration::from_secs(0),
            ..Default::default()
        }
    }

    pub fn add_result(&mut self, result: &PingResult) {
        self.total += 1;

        if result.success {
            self.successful += 1;

            if result.rtt < self.min_rtt {
                self.min_rtt = result.rtt;
            }

            if result.rtt > self.max_rtt {
                self.max_rtt = result.rtt;
            }

            self.total_rtt += result.rtt;
        } else {
            self.failed += 1;
        }
    }

    pub fn average_rtt(&self) -> Duration {
        if self.successful > 0 {
            self.total_rtt / self.successful
        } else {
            Duration::from_secs(0)
        }
    }
}
use crate::output::OutputManager;

impl TcpPing {
    pub async fn run(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let output_manager = OutputManager::new(&self.config.output);
        let mut statistics = Statistics::new();
        
        // Simple implementation: perform a single ping
        // Simple implementation: perform a single ping
        let result = self.ping(&self.config.hostname, self.config.port)?;
        // Output the result
        output_manager.output_probe(&result);
        
        // Update statistics
        statistics.add_result(&result);
        
        // Output final statistics
        output_manager.output_stats(&statistics);
        
        Ok(())
    }
}
