use crate::config::OutputConfig;
use crate::tcping::{ProbeResult, Statistics};
use colored::*;
// Removed unused imports to fix compilation warnings

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

    /// Output a single probe result
    pub fn output_probe(&self, result: &ProbeResult) {
        match self.format {
            OutputFormat::Human => self.output_probe_human(result),
            OutputFormat::Json => self.output_probe_json(result),
            OutputFormat::Minimal => self.output_probe_minimal(result),
            OutputFormat::Csv => self.output_probe_csv(result),
        }
    }

    /// Output final statistics
    pub fn output_stats(&self, stats: &Statistics) {
        match self.format {
            OutputFormat::Human => self.output_stats_human(stats),
            OutputFormat::Json => self.output_stats_json(stats),
            OutputFormat::Minimal => self.output_stats_minimal(stats),
            OutputFormat::Csv => self.output_stats_csv(stats),
        }
    }

    fn output_probe_human(&self, result: &ProbeResult) {
        if result.success {
            println!(
                "{} bytes from {}: {} time={:.2} ms",
                64, // Standard ICMP-like packet size for compatibility
                result.target_addr,
                "seq=1".bright_black(), // Placeholder sequence number
                result.rtt.unwrap_or(0.0)
            );
        } else {
            println!(
                "{}: {}",
                result.target_addr.to_string().red(),
                result.error.as_ref().unwrap_or(&"unknown error".to_string()).red()
            );
        }
    }

    fn output_probe_json(&self, result: &ProbeResult) {
        let json_output = serde_json::json!({
            "timestamp": chrono::Utc::now().to_rfc3339(),
            "success": result.success,
            "target": result.target_addr.to_string(),
            "rtt_ms": result.rtt,
            "error": result.error,
            "source_addr": result.source_addr.map(|addr| addr.to_string())
        });
        println!("{}", json_output);
    }

    fn output_probe_minimal(&self, result: &ProbeResult) {
        if result.success {
            println!("OK");
        } else {
            println!("FAIL");
        }
    }

    fn output_probe_csv(&self, result: &ProbeResult) {
        let timestamp = chrono::Utc::now().to_rfc3339();
        let success = if result.success { "true" } else { "false" };
        let rtt = result.rtt.unwrap_or(0.0);
        let default_error = String::new();
        let error = result.error.as_ref().unwrap_or(&default_error);

        println!("{},{},{},{:.2},{}", timestamp, success, result.target_addr, rtt, error);
    }

    fn output_stats_human(&self, stats: &Statistics) {
        println!("\n--- TCP Ping Statistics ---");
        println!("{} packets transmitted, {} received, {:.1}% packet loss",
            stats.total_probes.to_string().bold(),
            stats.successful_probes.to_string().green().bold(),
            stats.packet_loss
        );

        if let (Some(min), Some(max), Some(avg)) = (stats.min_rtt, stats.max_rtt, stats.avg_rtt) {
            println!("rtt min/avg/max = {:.2}/{:.2}/{:.2} ms",
                min.to_string().green(),
                avg.to_string().yellow(),
                max.to_string().red()
            );
        }

        // Duration tracking not implemented in current Statistics structure
        println!("Duration: Duration tracking not implemented");

        println!("Longest success streak: {}", stats.longest_success_streak);
        println!("Longest failure streak: {}", stats.longest_failure_streak);
    }

    fn output_stats_json(&self, stats: &Statistics) {
        let json_output = serde_json::json!({
            "statistics": {
                "total_probes": stats.total_probes,
                "successful_probes": stats.successful_probes,
                "failed_probes": stats.failed_probes,
                "packet_loss_percent": stats.packet_loss,
                "min_rtt_ms": stats.min_rtt,
                "max_rtt_ms": stats.max_rtt,
                "avg_rtt_ms": stats.avg_rtt,
                "longest_success_streak": stats.longest_success_streak,
                "longest_failure_streak": stats.longest_failure_streak,
                "duration_seconds": None::<f64>
            }
        });
        println!("{}", json_output);
    }

    fn output_stats_minimal(&self, stats: &Statistics) {
        println!("Transmitted: {}, Received: {}, Loss: {:.1}%",
            stats.total_probes, stats.successful_probes, stats.packet_loss);
    }

    fn output_stats_csv(&self, stats: &Statistics) {
        let timestamp = chrono::Utc::now().to_rfc3339();
        let min_rtt = stats.min_rtt.unwrap_or(0.0);
        let max_rtt = stats.max_rtt.unwrap_or(0.0);
        let avg_rtt = stats.avg_rtt.unwrap_or(0.0);

        println!("{},{},{},{},{:.1},{:.2},{:.2},{:.2},{},{}",
            timestamp,
            stats.total_probes,
            stats.successful_probes,
            stats.failed_probes,
            stats.packet_loss,
            min_rtt, avg_rtt, max_rtt,
            stats.longest_success_streak,
            stats.longest_failure_streak
        );
    }
}