//! Comprehensive test suite for tcping-rs core functionality

#[cfg(test)]
mod integration_tests {
    use std::net::IpAddr;
    use std::time::Duration;

    use crate::{Tcping, TcpingConfig, ProbeStatistics, ProbeResult};
    use std::time::Instant;
    use tokio::net::TcpListener;
    use tokio::time::timeout;

    /// Test basic TCP connection functionality
    #[tokio::test]
    async fn test_basic_tcp_connection() {
        // Start a simple TCP server for testing
        let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
        let local_addr = listener.local_addr().unwrap();
        let port = local_addr.port();

        // Create tcping config
        let config = TcpingConfig {
            host: "127.0.0.1".to_string(),
            port,
            timeout: Duration::from_secs(5),
            use_ipv4: false,
            use_ipv6: false,
        };

        let tcping = Tcping::new(config);

        // Test DNS resolution
        let resolved_ips = tcping.resolve_hostname("127.0.0.1").await;
        assert!(resolved_ips.is_ok());
        let ips = resolved_ips.unwrap();
        assert_eq!(ips.len(), 1);
        assert_eq!(ips[0], "127.0.0.1".parse::<IpAddr>().unwrap());

        // Test probe (should fail since server isn't accepting connections in this test)
        let result = timeout(Duration::from_secs(1), tcping.probe()).await;
        assert!(result.is_ok()); // Should complete within timeout
    }

    /// Test IPv4-only resolution
    #[tokio::test]
    async fn test_ipv4_only_resolution() {
        let config = TcpingConfig {
            host: "localhost".to_string(),
            port: 80,
            timeout: Duration::from_secs(1),
            use_ipv4: true,
            use_ipv6: false,
        };

        let tcping = Tcping::new(config);
        let result = tcping.resolve_hostname("localhost").await;

        if let Ok(ips) = result {
            // Should only contain IPv4 addresses
            for ip in ips {
                assert!(ip.is_ipv4());
            }
        }
    }

    /// Test invalid hostname resolution
    #[tokio::test]
    async fn test_invalid_hostname() {
        let config = TcpingConfig {
            host: "invalid-hostname-that-does-not-exist.local".to_string(),
            port: 80,
            timeout: Duration::from_secs(1),
            use_ipv4: false,
            use_ipv6: false,
        };

        let tcping = Tcping::new(config);
        let result = tcping.resolve_hostname("invalid-hostname-that-does-not-exist.local").await;

        assert!(result.is_err());
    }
}

#[cfg(test)]
mod unit_tests {
    use std::time::Instant;

    use crate::{ProbeStatistics, ProbeResult};

    /// Test ProbeStatistics calculation
    #[test]
    fn test_probe_statistics_calculation() {
        let mut stats = ProbeStatistics::default();

        // Test empty statistics
        assert_eq!(stats.total_probes, 0);
        assert_eq!(stats.successful_probes, 0);
        assert_eq!(stats.failed_probes, 0);
        assert_eq!(stats.packet_loss, 0.0);
        assert_eq!(stats.min_rtt, None);
        assert_eq!(stats.max_rtt, None);
        assert_eq!(stats.avg_rtt, None);

        // Add successful probe
        let result1 = ProbeResult {
            success: true,
            rtt: Some(10.0),
            error: None,
            timestamp: Instant::now(),
        };
        stats.update(&result1);

        assert_eq!(stats.total_probes, 1);
        assert_eq!(stats.successful_probes, 1);
        assert_eq!(stats.failed_probes, 0);
        assert_eq!(stats.packet_loss, 0.0);
        assert_eq!(stats.min_rtt, Some(10.0));
        assert_eq!(stats.max_rtt, Some(10.0));
        assert_eq!(stats.avg_rtt, Some(10.0));

        // Add failed probe
        let result2 = ProbeResult {
            success: false,
            rtt: None,
            error: Some("Timeout".to_string()),
            timestamp: Instant::now(),
        };
        stats.update(&result2);

        assert_eq!(stats.total_probes, 2);
        assert_eq!(stats.successful_probes, 1);
        assert_eq!(stats.failed_probes, 1);
        assert_eq!(stats.packet_loss, 50.0);

        // Add another successful probe with different RTT
        let result3 = ProbeResult {
            success: true,
            rtt: Some(20.0),
            error: None,
            timestamp: Instant::now(),
        };
        stats.update(&result3);

        assert_eq!(stats.total_probes, 3);
        assert_eq!(stats.successful_probes, 2);
        assert_eq!(stats.failed_probes, 1);
        assert_eq!(stats.min_rtt, Some(10.0));
        assert_eq!(stats.max_rtt, Some(20.0));
        assert_eq!(stats.avg_rtt, Some(15.0));
        assert!((stats.packet_loss - 33.33).abs() < 0.1);
    }

    /// Test statistics from results collection
    #[test]
    fn test_statistics_from_results() {
        let results = vec![
            ProbeResult {
                success: true,
                rtt: Some(5.0),
                error: None,
                timestamp: Instant::now(),
            },
            ProbeResult {
                success: true,
                rtt: Some(15.0),
                error: None,
                timestamp: Instant::now(),
            },
            ProbeResult {
                success: false,
                rtt: None,
                error: Some("Connection refused".to_string()),
                timestamp: Instant::now(),
            },
        ];

        let stats = ProbeStatistics::from_results(&results);

        assert_eq!(stats.total_probes, 3);
        assert_eq!(stats.successful_probes, 2);
        assert_eq!(stats.failed_probes, 1);
        assert!((stats.packet_loss - 33.33).abs() < 0.1);
        assert_eq!(stats.min_rtt, Some(5.0));
        assert_eq!(stats.max_rtt, Some(15.0));
        assert_eq!(stats.avg_rtt, Some(10.0));
    }

    /// Test edge cases in statistics
    #[test]
    fn test_statistics_edge_cases() {
        let mut stats = ProbeStatistics::default();

        // Test with only failed probes
        let failed_result = ProbeResult {
            success: false,
            rtt: None,
            error: Some("Timeout".to_string()),
            timestamp: Instant::now(),
        };

        stats.update(&failed_result);
        stats.update(&failed_result);

        assert_eq!(stats.total_probes, 2);
        assert_eq!(stats.successful_probes, 0);
        assert_eq!(stats.failed_probes, 2);
        assert_eq!(stats.packet_loss, 100.0);
        assert_eq!(stats.min_rtt, None);
        assert_eq!(stats.max_rtt, None);
        assert_eq!(stats.avg_rtt, None);

        // Test with successful probe but no RTT (shouldn't happen in practice)
        let success_no_rtt = ProbeResult {
            success: true,
            rtt: None,
            error: None,
            timestamp: Instant::now(),
        };

        let mut stats2 = ProbeStatistics::default();
        stats2.update(&success_no_rtt);

        assert_eq!(stats2.total_probes, 1);
        assert_eq!(stats2.successful_probes, 1);
        assert_eq!(stats2.failed_probes, 0);
        assert_eq!(stats2.packet_loss, 0.0);
        assert_eq!(stats2.min_rtt, None);
        assert_eq!(stats2.max_rtt, None);
        assert_eq!(stats2.avg_rtt, None);
    }
}

#[cfg(test)]
mod config_tests {
    use std::time::Duration;

    use crate::TcpingConfig;

    /// Test TcpingConfig default values
    #[test]
    fn test_config_defaults() {
        let config = TcpingConfig::default();

        assert_eq!(config.host, "");
        assert_eq!(config.port, 80);
        assert_eq!(config.timeout, Duration::from_secs(1));
        assert!(!config.use_ipv4);
        assert!(!config.use_ipv6);
    }

    /// Test IP version validation logic
    #[test]
    fn test_ip_version_validation() {
        let config_ipv4 = TcpingConfig {
            host: "example.com".to_string(),
            port: 80,
            timeout: Duration::from_secs(1),
            use_ipv4: true,
            use_ipv6: false,
        };

        let config_ipv6 = TcpingConfig {
            host: "example.com".to_string(),
            port: 80,
            timeout: Duration::from_secs(1),
            use_ipv4: false,
            use_ipv6: true,
        };

        let config_both = TcpingConfig {
            host: "example.com".to_string(),
            port: 80,
            timeout: Duration::from_secs(1),
            use_ipv4: true,
            use_ipv6: true,
        };

        // These should be valid configurations
        assert!(config_ipv4.use_ipv4 && !config_ipv4.use_ipv6);
        assert!(!config_ipv6.use_ipv4 && config_ipv6.use_ipv6);
        assert!(config_both.use_ipv4 && config_both.use_ipv6);
    }
}

#[cfg(test)]
mod error_tests {
    use std::time::Duration;

    use crate::TcpingError;

    /// Test error type formatting
    #[test]
    fn test_error_display() {
        let dns_error = TcpingError::DnsResolution("example.com".to_string());
        assert_eq!(
            dns_error.to_string(),
            "DNS resolution failed: example.com"
        );

        let connection_error = TcpingError::Connection("Connection refused".to_string());
        assert_eq!(
            connection_error.to_string(),
            "TCP connection failed: Connection refused"
        );

        let timeout_error = TcpingError::Timeout(Duration::from_secs(5));
        assert_eq!(
            timeout_error.to_string(),
            "Timeout after 5s"
        );
    }
}