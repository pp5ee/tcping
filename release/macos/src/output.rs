use crate::config::OutputConfig;
use crate::tcping::{PingResult, Statistics};
use colored::*;

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
    pub fn output_probe(&self, result: &PingResult) {
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

    fn output_probe_human(&self, result: &PingResult) {
        if result.success {
            println!(
                "time={:.2} ms",
                result.rtt.as_secs_f64() * 1000.0
            );
        } else {
            println!(
                "{}",
                result.error.as_ref().unwrap_or(&"unknown error".to_string()).red()
            );
        }
    }

    fn output_probe_json(&self, result: &PingResult) {
        let json_output = serde_json::json!({
            "timestamp": chrono::Utc::now().to_rfc3339(),
            "success": result.success,
            "rtt_ms": result.rtt,
            "error": result.error,
        });
        println!("{}", json_output);
    }

    fn output_probe_minimal(&self, result: &PingResult) {
        if result.success {
            println!("OK");
        } else {
            println!("FAIL");
        }
    }

    fn output_probe_csv(&self, result: &PingResult) {
        let timestamp = chrono::Utc::now().to_rfc3339();
        let success = if result.success { "true" } else { "false" };
        let rtt = result.rtt.as_secs_f64();
        let default_error = String::new();
        let error = result.error.as_ref().unwrap_or(&default_error);
        
        println!("{},{},{:.2},{}", timestamp, success, rtt, error);
    }

    fn output_stats_human(&self, stats: &Statistics) {
        println!("\n--- TCP Ping Statistics ---");
        println!("{} packets transmitted, {} received, {:.1}% packet loss",
            stats.total.to_string().bold(),
            stats.successful.to_string().green().bold(),
            (stats.failed as f64 / stats.total as f64) * 100.0
        );

        if stats.total > 0 {
            let min = stats.min_rtt.as_secs_f64() * 1000.0;
            let max = stats.max_rtt.as_secs_f64() * 1000.0;
            let avg = stats.average_rtt().as_secs_f64() * 1000.0;
            
            println!("rtt min/avg/max = {:.2}/{:.2}/{:.2} ms",
                min,
                avg,
                max
            );
        }
        // Duration tracking not implemented in current Statistics structure
        println!("Duration: Duration tracking not implemented");
    }

    fn output_stats_json(&self, stats: &Statistics) {
        let json_output = serde_json::json!({
            "statistics": {
                "total_probes": stats.total,
                "successful_probes": stats.successful,
                "failed_probes": stats.failed,
                "packet_loss_percent": (stats.failed as f64 / stats.total as f64) * 100.0,
                "min_rtt_ms": stats.min_rtt,
                "max_rtt_ms": stats.max_rtt,
                "avg_rtt_ms": stats.average_rtt(),
                "duration_seconds": None::<f64>
            }
        });
        println!("{}", json_output);
    }

    fn output_stats_minimal(&self, stats: &Statistics) {
        println!("Transmitted: {}, Received: {}, Loss: {:.1}%",
            stats.total, stats.successful, (stats.failed as f64 / stats.total as f64) * 100.0);
    }

    fn output_stats_csv(&self, stats: &Statistics) {
        let timestamp = chrono::Utc::now().to_rfc3339();
        let min_rtt = stats.min_rtt.as_secs_f64() * 1000.0;
        let max_rtt = stats.max_rtt.as_secs_f64() * 1000.0;
        let avg_rtt = stats.average_rtt().as_secs_f64() * 1000.0;
        
        println!("{},{},{:.1},{:.2},{:.2},{:.2}",
            timestamp,
            stats.total,
            (stats.failed as f64 / stats.total as f64) * 100.0,
            min_rtt,
            avg_rtt,
            max_rtt
        );
    }
}
