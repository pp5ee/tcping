// Unit tests for tcping Rust implementation

#[cfg(test)]
mod tests {
    use std::net::{Ipv4Addr, Ipv6Addr, SocketAddr};
    use tcping::{parse_host_port, validate_args, OutputFormat};

    #[test]
    fn test_parse_host_port() {
        // Test basic host:port format
        let result = parse_host_port("google.com:80");
        assert!(result.is_ok());
        let (host, port) = result.unwrap();
        assert_eq!(host, "google.com");
        assert_eq!(port, 80);

        // Test IPv4 address
        let result = parse_host_port("192.168.1.1:443");
        assert!(result.is_ok());
        let (host, port) = result.unwrap();
        assert_eq!(host, "192.168.1.1");
        assert_eq!(port, 443);

        // Test IPv6 address
        let result = parse_host_port("[::1]:8080");
        assert!(result.is_ok());
        let (host, port) = result.unwrap();
        assert_eq!(host, "::1");
        assert_eq!(port, 8080);

        // Test invalid formats
        assert!(parse_host_port("invalid").is_err());
        assert!(parse_host_port("host:port").is_err());
        assert!(parse_host_port("host:").is_err());
        assert!(parse_host_port(":80").is_err());
    }

    #[test]
    fn test_validate_args() {
        // Test valid arguments
        let args = vec!["tcping", "google.com", "80"];
        assert!(validate_args(&args).is_ok());

        // Test missing arguments
        let args = vec!["tcping", "google.com"];
        assert!(validate_args(&args).is_err());

        // Test invalid port
        let args = vec!["tcping", "google.com", "not_a_port"];
        assert!(validate_args(&args).is_err());

        // Test port out of range
        let args = vec!["tcping", "google.com", "70000"];
        assert!(validate_args(&args).is_err());
    }

    #[test]
    fn test_output_format_parsing() {
        // Test JSON format
        let format = OutputFormat::from_str("json");
        assert_eq!(format, OutputFormat::Json);

        // Test CSV format
        let format = OutputFormat::from_str("csv");
        assert_eq!(format, OutputFormat::Csv);

        // Test default format
        let format = OutputFormat::from_str("default");
        assert_eq!(format, OutputFormat::Default);

        // Test invalid format
        let format = OutputFormat::from_str("invalid");
        assert_eq!(format, OutputFormat::Default);
    }

    #[test]
    fn test_socket_addr_parsing() {
        // Test IPv4 socket address
        let addr: SocketAddr = "192.168.1.1:80".parse().unwrap();
        assert_eq!(addr.ip(), Ipv4Addr::new(192, 168, 1, 1));
        assert_eq!(addr.port(), 80);

        // Test IPv6 socket address
        let addr: SocketAddr = "[::1]:8080".parse().unwrap();
        assert_eq!(addr.port(), 8080);

        // Test invalid socket address
        assert!("invalid".parse::<SocketAddr>().is_err());
    }

    #[test]
    fn test_duration_parsing() {
        // Test valid duration
        let duration = tcping::parse_duration("5s");
        assert!(duration.is_ok());
        assert_eq!(duration.unwrap().as_secs(), 5);

        // Test invalid duration
        let duration = tcping::parse_duration("invalid");
        assert!(duration.is_err());
    }

    #[test]
    fn test_count_validation() {
        // Test valid count
        assert!(tcping::validate_count(1).is_ok());
        assert!(tcping::validate_count(100).is_ok());

        // Test invalid count
        assert!(tcping::validate_count(0).is_err());
        assert!(tcping::validate_count(-1).is_err());
    }

    #[test]
    fn test_timeout_validation() {
        // Test valid timeout
        assert!(tcping::validate_timeout(1).is_ok());
        assert!(tcping::validate_timeout(30).is_ok());

        // Test invalid timeout
        assert!(tcping::validate_timeout(0).is_err());
        assert!(tcping::validate_timeout(-1).is_err());
    }

    #[test]
    fn test_interval_validation() {
        // Test valid interval
        assert!(tcping::validate_interval(1).is_ok());
        assert!(tcping::validate_interval(10).is_ok());

        // Test invalid interval
        assert!(tcping::validate_interval(0).is_err());
        assert!(tcping::validate_interval(-1).is_err());
    }

    #[test]
    fn test_ip_version_parsing() {
        // Test IPv4
        let version = tcping::parse_ip_version("4");
        assert_eq!(version, tcping::IpVersion::V4);

        // Test IPv6
        let version = tcping::parse_ip_version("6");
        assert_eq!(version, tcping::IpVersion::V6);

        // Test auto (default)
        let version = tcping::parse_ip_version("auto");
        assert_eq!(version, tcping::IpVersion::Auto);

        // Test invalid
        let version = tcping::parse_ip_version("invalid");
        assert_eq!(version, tcping::IpVersion::Auto);
    }

    #[test]
    fn test_statistics_calculation() {
        let mut stats = tcping::Statistics::new();

        // Add some sample data
        stats.add_success(100.0);
        stats.add_success(200.0);
        stats.add_success(150.0);
        stats.add_failure();

        // Test statistics
        assert_eq!(stats.total(), 4);
        assert_eq!(stats.successful(), 3);
        assert_eq!(stats.failed(), 1);
        assert_eq!(stats.success_rate(), 75.0);

        // Test RTT calculations
        let rtt_stats = stats.rtt_statistics();
        assert_eq!(rtt_stats.min, 100.0);
        assert_eq!(rtt_stats.max, 200.0);
        assert_eq!(rtt_stats.avg, 150.0);
    }

    #[test]
    fn test_color_output_detection() {
        // Test color detection (should return false in test environment)
        let supports_color = tcping::supports_color();
        assert!(!supports_color); // Typically false in test environment
    }

    #[test]
    fn test_timestamp_generation() {
        let timestamp = tcping::generate_timestamp();
        assert!(!timestamp.is_empty());
        // Should contain date and time components
        assert!(timestamp.contains('-'));
        assert!(timestamp.contains(':'));
    }
}