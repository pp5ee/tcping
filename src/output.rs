use crate::tcping::{ProbeResult, Statistics};
use crate::config::OutputConfig;
use colored::*;
use serde::{Deserialize, Serialize};
use std::time::Instant;

/// Output formatter trait for different output formats
pub trait OutputFormatter {
    /// Format a single probe result
    fn format_probe(&self, result: &ProbeResult) -> String;

    /// Format final statistics
    fn format_stats(&self, stats: &Statistics) -> String;
}

/// Console output formatter
pub struct ConsoleFormatter {
    config: OutputConfig,
}

impl ConsoleFormatter {
    pub fn new(config: OutputConfig) -> Self {
        Self { config }
    }
}

impl OutputFormatter for ConsoleFormatter {
    fn format_probe(&self, result: &ProbeResult) -> String {
        let timestamp = if self.config.timestamps {
            format!("[{}] ", chrono::Local::now().format("%Y-%m-%d %H:%M:%S%.3f"))
        } else {
            String::new()
        };

        let target_str = format!("{}:{}", result.target_addr.ip(), result.target_addr.port());

        if result.success {
            let rtt = result.rtt.unwrap_or(0.0);
            let status = if self.config.color {
                format!("{} {:.2}ms", "✓".green(), rtt)
            } else {
                format!("✓ {:.2}ms", rtt)
            };

            if let Some(source_addr) = result.source_addr {
                if self.config.show_source_address {
                    format!("{}{} from {}: {}", timestamp, target_str, source_addr, status)
                } else {
                    format!("{}{}: {}", timestamp, target_str, status)
                }
            } else {
                format!("{}{}: {}", timestamp, target_str, status)
            }
        } else {
            let error = result.error.as_deref().unwrap_or("Unknown error");
            let status = if self.config.color {
                format!("{} {}", "✗".red(), error)
            } else {
                format!("✗ {}", error)
            };

            if self.config.show_failures_only || !self.config.show_failures_only {
                format!("{}{}: {}", timestamp, target_str, status)
            } else {
                String::new()
            }
        }
    }

    fn format_stats(&self, stats: &Statistics) -> String {
        let duration = if let Some(start_time) = stats.start_time {
            start_time.elapsed()
        } else {
            return "No statistics available".to_string();
        };

        let mut output = vec![];

        if self.config.color {
            output.push(format!("{} TCP Ping Statistics", "=".repeat(30).bold()));
        } else {
            output.push(format!("{} TCP Ping Statistics", "=".repeat(30)));
        }

        output.push(format!("Probes sent: {}", stats.total_probes));
        output.push(format!("Successful: {} ({:.1}%)", stats.successful_probes, 100.0 - stats.packet_loss));
        output.push(format!("Failed: {} ({:.1}% loss)", stats.failed_probes, stats.packet_loss));

        if let Some(avg_rtt) = stats.avg_rtt {
            output.push(format!("Average RTT: {:.2}ms", avg_rtt));
        }

        if let Some(min_rtt) = stats.min_rtt {
            output.push(format!("Minimum RTT: {:.2}ms", min_rtt));
        }

        if let Some(max_rtt) = stats.max_rtt {
            output.push(format!("Maximum RTT: {:.2}ms", max_rtt));
        }

        output.push(format!("Duration: {:.2}s", duration.as_secs_f64()));
        output.push(format!("Longest success streak: {}", stats.longest_success_streak));
        output.push(format!("Longest failure streak: {}", stats.longest_failure_streak));

        output.join("\n")
    }
}

/// JSON output formatter
pub struct JsonFormatter {
    config: OutputConfig,
}

impl JsonFormatter {
    pub fn new(config: OutputConfig) -> Self {
        Self { config }
    }
}

#[derive(Serialize, Deserialize)]
struct JsonProbeResult {
    success: bool,
    rtt: Option<f64>,
    error: Option<String>,
    source_addr: Option<String>,
    timestamp: String,
    target_addr: String,
}

#[derive(Serialize, Deserialize)]
struct JsonStatistics {
    total_probes: u32,
    successful_probes: u32,
    failed_probes: u32,
    min_rtt: Option<f64>,
    max_rtt: Option<f64>,
    avg_rtt: Option<f64>,
    packet_loss: f64,
    duration_seconds: f64,
    longest_success_streak: u32,
    longest_failure_streak: u32,
}

impl OutputFormatter for JsonFormatter {
    fn format_probe(&self, result: &ProbeResult) -> String {
        let json_result = JsonProbeResult {
            success: result.success,
            rtt: result.rtt,
            error: result.error.clone(),
            source_addr: result.source_addr.map(|addr| addr.to_string()),
            timestamp: chrono::Local::now().to_rfc3339(),
            target_addr: result.target_addr.to_string(),
        };

        if self.config.pretty {
            serde_json::to_string_pretty(&json_result).unwrap_or_else(|_| "{}".to_string())
        } else {
            serde_json::to_string(&json_result).unwrap_or_else(|_| "{}".to_string())
        }
    }

    fn format_stats(&self, stats: &Statistics) -> String {
        let duration = if let Some(start_time) = stats.start_time {
            start_time.elapsed().as_secs_f64()
        } else {
            0.0
        };

        let json_stats = JsonStatistics {
            total_probes: stats.total_probes,
            successful_probes: stats.successful_probes,
            failed_probes: stats.failed_probes,
            min_rtt: stats.min_rtt,
            max_rtt: stats.max_rtt,
            avg_rtt: stats.avg_rtt,
            packet_loss: stats.packet_loss,
            duration_seconds: duration,
            longest_success_streak: stats.longest_success_streak,
            longest_failure_streak: stats.longest_failure_streak,
        };

        if self.config.pretty {
            serde_json::to_string_pretty(&json_stats).unwrap_or_else(|_| "{}".to_string())
        } else {
            serde_json::to_string(&json_stats).unwrap_or_else(|_| "{}".to_string())
        }
    }
}

/// Output manager that handles multiple output formats
pub struct OutputManager {
    formatters: Vec<Box<dyn OutputFormatter>>,
}

impl OutputManager {
    pub fn new(config: &OutputConfig) -> Self {
        let mut formatters: Vec<Box<dyn OutputFormatter>> = Vec::new();

        // Always include console output
        formatters.push(Box::new(ConsoleFormatter::new(config.clone())));

        // Add JSON output if requested
        if config.json {
            formatters.push(Box::new(JsonFormatter::new(config.clone())));
        }

        Self { formatters }
    }

    /// Output a probe result using all configured formatters
    pub fn output_probe(&self, result: &ProbeResult) {
        for formatter in &self.formatters {
            let output = formatter.format_probe(result);
            if !output.is_empty() {
                println!("{}", output);
            }
        }
    }

    /// Output final statistics using all configured formatters
    pub fn output_stats(&self, stats: &Statistics) {
        for formatter in &self.formatters {
            let output = formatter.format_stats(stats);
            if !output.is_empty() {
                println!("{}", output);
            }
        }
    }
}