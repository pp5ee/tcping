use crate::config::OutputConfig;
use crate::tcping::{PingResult, TcpPingStats};
use colored::*;
use chrono::{DateTime, Utc};
use std::time::Duration;

/// Output format options
#[derive(Debug, Clone, PartialEq)]
pub enum OutputFormat {
    /// Human-readable colored output (default)
    Human,
    /// JSON format for machine processing
    Json,
    /// Minimal output (just success/failure)
    Minimal,
    /// CSV format for data analysis
    Csv,
}

impl Default for OutputFormat {
    fn default() -> Self {
        Self::Human
    }
}

/// Manages output formatting for TCP ping results
pub struct OutputManager {
    config: OutputConfig,
    format: OutputFormat,
}

impl OutputManager {
    pub fn new(config: &OutputConfig) -> Self {
        let format = if config.json {
            OutputFormat::Json
        } else if config.csv_file.is_some() {
            OutputFormat::Csv
        } else {
            OutputFormat::Human
        };

        Self {
            config: config.clone(),
            format,
        }
    }

    /// Output start message
    pub fn output_start(&self, hostname: &str, port: u16) {
        match self.format {
            OutputFormat::Human => self.output_start_human(hostname, port),
            OutputFormat::Json => {}, // JSON doesn't need start message
            OutputFormat::Minimal => {}, // Minimal doesn't need start message
            OutputFormat::Csv => {}, // CSV doesn't need start message
        }
    }

    /// Output a single probe result
    pub fn output_probe(&self, result: &PingResult, stats: &TcpPingStats) {
        match self.format {
            OutputFormat::Human => self.output_probe_human(result, stats),
            OutputFormat::Json => self.output_probe_json(result, stats),
            OutputFormat::Minimal => self.output_probe_minimal(result, stats),
            OutputFormat::Csv => self.output_probe_csv(result, stats),
        }
    }

    /// Output final statistics
    pub fn output_stats(&self, stats: &TcpPingStats) {
        match self.format {
            OutputFormat::Human => self.output_stats_human(stats),
            OutputFormat::Json => self.output_stats_json(stats),
            OutputFormat::Minimal => self.output_stats_minimal(stats),
            OutputFormat::Csv => self.output_stats_csv(stats),
        }
    }

    fn output_start_human(&self, hostname: &str, port: u16) {
        if self.config.timestamps {
            let timestamp = chrono::Local::now().format("%Y-%m-%d %H:%M:%S%.3f").to_string();
            println!("[{}] TCPING {}:{}", timestamp, hostname, port);
        } else {
            println!("TCPING {}:{}", hostname, port);
        }
    }

    fn output_probe_human(&self, result: &PingResult, stats: &TcpPingStats) {
        // Skip if only showing failures and this is a success
        if self.config.show_failures_only && result.success {
            return;
        }

        let timestamp = if self.config.timestamps {
            format!("[{}] ", chrono::Local::now().format("%Y-%m-%d %H:%M:%S%.3f"))
        } else {
            String::new()
        };

        if result.success {
            let rtt_ms = result.rtt.as_secs_f64() * 1000.0;
            let source_addr = if self.config.show_source_address {
                result.source_addr.as_ref().map_or(String::new(), |addr| format!(" from {}", addr))
            } else {
                String::new()
            };

            if self.config.color {
                println!("{}{}Reply from {}:{} time={:.2} ms{}",
                    timestamp,
                    "✓ ".green(),
                    stats.hostname_changes.last().map_or("unknown", |hc| &hc.addr),
                    "unknown", // Port would need to be tracked separately
                    rtt_ms,
                    source_addr
                );
            } else {
                println!("{}Reply from {}:{} time={:.2} ms{}",
                    timestamp,
                    stats.hostname_changes.last().map_or("unknown", |hc| &hc.addr),
                    "unknown", // Port would need to be tracked separately
                    rtt_ms,
                    source_addr
                );
            }
        } else {
            if self.config.color {
                println!("{}{} {}",
                    timestamp,
                    "✗ ".red(),
                    result.error.as_ref().unwrap_or(&"unknown error".to_string())
                );
            } else {
                println!("{} {}",
                    timestamp,
                    result.error.as_ref().unwrap_or(&"unknown error".to_string())
                );
            }
        }
    }

    fn output_probe_json(&self, result: &PingResult, _stats: &TcpPingStats) {
        let mut json_output = serde_json::json!({
            "timestamp": chrono::Utc::now().to_rfc3339(),
            "success": result.success,
            "rtt_ms": result.rtt.as_secs_f64() * 1000.0,
        });

        if let Some(error) = &result.error {
            json_output["error"] = serde_json::Value::String(error.clone());
        }

        if let Some(source_addr) = &result.source_addr {
            json_output["source_address"] = serde_json::Value::String(source_addr.clone());
        }

        if self.config.pretty {
            println!("{}", serde_json::to_string_pretty(&json_output).unwrap());
        } else {
            println!("{}", json_output);
        }
    }

    fn output_probe_minimal(&self, result: &PingResult, _stats: &TcpPingStats) {
        if result.success {
            println!("OK");
        } else {
            println!("FAIL");
        }
    }

    fn output_probe_csv(&self, result: &PingResult, _stats: &TcpPingStats) {
        let timestamp = chrono::Utc::now().to_rfc3339();
        let success = if result.success { "true" } else { "false" };
        let rtt = result.rtt.as_secs_f64() * 1000.0;
        let default_error = String::new();
        let error = result.error.as_ref().unwrap_or(&default_error);

        println!("{},{},{:.2},{}", timestamp, success, rtt, error);
    }

    fn output_stats_human(&self, stats: &TcpPingStats) {
        println!("\n--- TCP Ping Statistics ---");

        let total_probes = stats.total_successful_probes + stats.total_unsuccessful_probes;
        let packet_loss = if total_probes > 0 {
            (stats.total_unsuccessful_probes as f64 / total_probes as f64) * 100.0
        } else {
            0.0
        };

        println!("{} probes transmitted, {} successful, {} failed, {:.1}% packet loss",
            total_probes.to_string().bold(),
            stats.total_successful_probes.to_string().green().bold(),
            stats.total_unsuccessful_probes.to_string().red().bold(),
            packet_loss
        );

        // RTT statistics
        if stats.rtt_results.has_results {
            println!("rtt min/avg/max = {:.2}/{:.2}/{:.2} ms",
                stats.rtt_results.min,
                stats.rtt_results.average,
                stats.rtt_results.max
            );
        }

        // Duration information
        if let Some(end_time) = stats.end_time {
            let duration = end_time.signed_duration_since(stats.start_time);
            println!("Duration: {:.2} seconds", duration.num_milliseconds() as f64 / 1000.0);
        }

        // Uptime/downtime statistics
        if let Some(uptime_start) = stats.start_of_uptime {
            let uptime = Utc::now().signed_duration_since(uptime_start);
            println!("Current uptime: {:.2} seconds", uptime.num_milliseconds() as f64 / 1000.0);
        }

        if let Some(downtime_start) = stats.start_of_downtime {
            let downtime = Utc::now().signed_duration_since(downtime_start);
            println!("Current downtime: {:.2} seconds", downtime.num_milliseconds() as f64 / 1000.0);
        }

        // Longest uptime/downtime
        if stats.longest_uptime.duration > Duration::default() {
            println!("Longest uptime: {:.2} seconds", stats.longest_uptime.duration.as_secs_f64());
        }

        if stats.longest_downtime.duration > Duration::default() {
            println!("Longest downtime: {:.2} seconds", stats.longest_downtime.duration.as_secs_f64());
        }

        // Hostname changes
        if stats.hostname_changes.len() > 1 {
            println!("Hostname changes: {} ({} retries)",
                stats.hostname_changes.len() - 1,
                stats.retried_hostname_lookups
            );
        }
    }

    fn output_stats_json(&self, stats: &TcpPingStats) {
        let total_probes = stats.total_successful_probes + stats.total_unsuccessful_probes;
        let packet_loss = if total_probes > 0 {
            (stats.total_unsuccessful_probes as f64 / total_probes as f64) * 100.0
        } else {
            0.0
        };

        let duration = if let Some(end_time) = stats.end_time {
            end_time.signed_duration_since(stats.start_time).num_milliseconds() as f64 / 1000.0
        } else {
            0.0
        };

        let mut json_output = serde_json::json!({
            "statistics": {
                "total_probes": total_probes,
                "successful_probes": stats.total_successful_probes,
                "failed_probes": stats.total_unsuccessful_probes,
                "packet_loss_percent": packet_loss,
                "duration_seconds": duration,
                "hostname_changes": stats.hostname_changes.len(),
                "retried_hostname_lookups": stats.retried_hostname_lookups
            }
        });

        if stats.rtt_results.has_results {
            json_output["statistics"]["min_rtt_ms"] = serde_json::Value::from(stats.rtt_results.min);
            json_output["statistics"]["max_rtt_ms"] = serde_json::Value::from(stats.rtt_results.max);
            json_output["statistics"]["avg_rtt_ms"] = serde_json::Value::from(stats.rtt_results.average);
        }

        if self.config.pretty {
            println!("{}", serde_json::to_string_pretty(&json_output).unwrap());
        } else {
            println!("{}", json_output);
        }
    }

    fn output_stats_minimal(&self, stats: &TcpPingStats) {
        let total_probes = stats.total_successful_probes + stats.total_unsuccessful_probes;
        let packet_loss = if total_probes > 0 {
            (stats.total_unsuccessful_probes as f64 / total_probes as f64) * 100.0
        } else {
            0.0
        };

        println!("Transmitted: {}, Successful: {}, Failed: {}, Loss: {:.1}%",
            total_probes, stats.total_successful_probes, stats.total_unsuccessful_probes, packet_loss);
    }

    fn output_stats_csv(&self, stats: &TcpPingStats) {
        let timestamp = chrono::Utc::now().to_rfc3339();
        let total_probes = stats.total_successful_probes + stats.total_unsuccessful_probes;
        let packet_loss = if total_probes > 0 {
            (stats.total_unsuccessful_probes as f64 / total_probes as f64) * 100.0
        } else {
            0.0
        };

        let min_rtt = if stats.rtt_results.has_results { stats.rtt_results.min } else { 0.0 };
        let max_rtt = if stats.rtt_results.has_results { stats.rtt_results.max } else { 0.0 };
        let avg_rtt = if stats.rtt_results.has_results { stats.rtt_results.average } else { 0.0 };

        println!("{},{},{},{},{:.1},{:.2},{:.2},{:.2},{},{}",
            timestamp,
            total_probes,
            stats.total_successful_probes,
            stats.total_unsuccessful_probes,
            packet_loss,
            min_rtt,
            avg_rtt,
            max_rtt,
            stats.hostname_changes.len(),
            stats.retried_hostname_lookups
        );
    }
}
