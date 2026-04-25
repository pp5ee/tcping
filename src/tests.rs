#[cfg(test)]
mod tests {
    use super::*;
    use std::net::{IpAddr, Ipv4Addr, SocketAddr};
    use std::time::Duration;

    #[test]
    fn test_config_creation() {
        let config = Config {
            target: SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 80),
            count: Some(10),
            interval: Duration::from_secs(1),
            timeout: Duration::from_secs(5),
            output: OutputConfig::default(),
        };

        assert_eq!(config.target.port(), 80);
        assert_eq!(config.count, Some(10));
        assert_eq!(config.interval, Duration::from_secs(1));
        assert_eq!(config.timeout, Duration::from_secs(5));
    }

    #[test]
    fn test_probe_result_creation() {
        let result = ProbeResult {
            target_addr: SocketAddr::new(IpAddr::V4(Ipv4Addr::new(8, 8, 8, 8)), 53),
            source_addr: Some(SocketAddr::new(IpAddr::V4(Ipv4Addr::new(192, 168, 1, 1)), 0)),
            success: true,
            rtt: Some(15.5),
            error: None,
        };

        assert!(result.success);
        assert_eq!(result.rtt, Some(15.5));
        assert!(result.error.is_none());
    }

    #[test]
    fn test_statistics_calculation() {
        let mut stats = Statistics::new();

        // Simulate some successful probes
        for _ in 0..5 {
            stats.record_success(Some(10.0));
        }

        // Simulate some failed probes
        for _ in 0..2 {
            stats.record_failure();
        }

        assert_eq!(stats.total_probes, 7);
        assert_eq!(stats.successful_probes, 5);
        assert_eq!(stats.failed_probes, 2);
        assert_eq!(stats.packet_loss, (2.0 / 7.0) * 100.0);
        assert_eq!(stats.avg_rtt, Some(10.0));
        assert_eq!(stats.min_rtt, Some(10.0));
        assert_eq!(stats.max_rtt, Some(10.0));
    }

    #[test]
    fn test_output_config_default() {
        let config = OutputConfig::default();

        assert!(!config.json);
        assert!(!config.pretty);
        assert!(!config.timestamps);
        assert!(config.color);
        assert!(!config.show_failures_only);
        assert!(!config.show_source_address);
    }

    #[test]
    fn test_console_formatter() {
        let config = OutputConfig::default();
        let formatter = ConsoleFormatter::new(config);

        let result = ProbeResult {
            target_addr: SocketAddr::new(IpAddr::V4(Ipv4Addr::new(8, 8, 8, 8)), 53),
            source_addr: None,
            success: true,
            rtt: Some(15.5),
            error: None,
        };

        let output = formatter.format_probe(&result);
        assert!(output.contains("8.8.8.8:53"));
        assert!(output.contains("15.50ms"));
    }

    #[test]
    fn test_json_formatter() {
        let config = OutputConfig {
            json: true,
            pretty: false,
            ..Default::default()
        };

        let formatter = JsonFormatter::new(config);

        let result = ProbeResult {
            target_addr: SocketAddr::new(IpAddr::V4(Ipv4Addr::new(8, 8, 8, 8)), 53),
            source_addr: None,
            success: true,
            rtt: Some(15.5),
            error: None,
        };

        let output = formatter.format_probe(&result);
        assert!(output.contains("\"success\":true"));
        assert!(output.contains("\"rtt\":15.5"));
    }

    #[test]
    fn test_cli_validation() {
        let cli = Cli {
            target: "google.com:80".to_string(),
            count: Some(5),
            interval: Some(1),
            timeout: Some(3),
            json: false,
            pretty: false,
            timestamps: false,
            color: true,
            show_failures_only: false,
            show_source_address: false,
        };

        assert!(cli.validate().is_ok());
    }

    #[test]
    fn test_cli_invalid_target() {
        let cli = Cli {
            target: "invalid-target".to_string(),
            count: Some(5),
            interval: Some(1),
            timeout: Some(3),
            json: false,
            pretty: false,
            timestamps: false,
            color: true,
            show_failures_only: false,
            show_source_address: false,
        };

        assert!(cli.validate().is_err());
    }
}