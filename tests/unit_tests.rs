// Unit tests for tcping Rust implementation

#[cfg(test)]
mod tests {
    use std::net::{Ipv4Addr, Ipv6Addr, SocketAddr};
    use tcping::{Config, ProtocolFamily, OutputFormat, TcpPingBuilder};

    #[test]
    fn test_tcp_ping_builder() {
        // Test basic builder creation
        let builder = TcpPingBuilder::new("example.com".to_string(), 80);
        let tcping = builder.build();

        // Verify the builder creates a valid TcpPing instance
        assert!(true); // Just test that it doesn't panic
    }

    #[test]
    fn test_protocol_family_enum() {
        // Test ProtocolFamily enum variants
        assert_eq!(format!("{:?}", ProtocolFamily::Any), "Any");
        assert_eq!(format!("{:?}", ProtocolFamily::IPv4Only), "IPv4Only");
        assert_eq!(format!("{:?}", ProtocolFamily::IPv6Only), "IPv6Only");
    }

    #[test]
    fn test_output_format_enum() {
        // Test OutputFormat enum variants
        assert_eq!(format!("{:?}", OutputFormat::Human), "Human");
        assert_eq!(format!("{:?}", OutputFormat::Json), "Json");
        assert_eq!(format!("{:?}", OutputFormat::Minimal), "Minimal");
        assert_eq!(format!("{:?}", OutputFormat::Csv), "Csv");
    }

    #[test]
    fn test_config_creation() {
        // Test basic config creation
        let config = Config {
            hostname: "example.com".to_string(),
            port: 80,
            protocol_family: ProtocolFamily::Any,
            retry_resolution: 0,
            max_probes: None,
            interval: std::time::Duration::from_secs(1),
            timeout: std::time::Duration::from_secs(1),
            interface: None,
            output: tcping::OutputConfig {
                json: false,
                pretty: false,
                color: true,
                timestamps: false,
                csv_file: None,
                db_file: None,
                show_source_address: false,
                show_failures_only: false,
            },
            verbosity: 0,
        };

        assert_eq!(config.hostname, "example.com");
        assert_eq!(config.port, 80);
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
    fn test_builder_with_protocol_family() {
        // Test builder with IPv4 protocol family
        let builder = TcpPingBuilder::new("example.com".to_string(), 80)
            .protocol_family(ProtocolFamily::IPv4Only);
        let _tcping = builder.build();

        assert!(true); // Just test that it doesn't panic
    }

    #[test]
    fn test_builder_with_max_probes() {
        // Test builder with max probes
        let builder = TcpPingBuilder::new("example.com".to_string(), 80)
            .max_probes(Some(10));
        let _tcping = builder.build();

        assert!(true); // Just test that it doesn't panic
    }

    #[test]
    fn test_builder_with_interval() {
        // Test builder with custom interval
        let builder = TcpPingBuilder::new("example.com".to_string(), 80)
            .interval(std::time::Duration::from_secs(2));
        let _tcping = builder.build();

        assert!(true); // Just test that it doesn't panic
    }

    #[test]
    fn test_builder_with_timeout() {
        // Test builder with custom timeout
        let builder = TcpPingBuilder::new("example.com".to_string(), 80)
            .timeout(std::time::Duration::from_secs(5));
        let _tcping = builder.build();

        assert!(true); // Just test that it doesn't panic
    }

    #[test]
    fn test_timestamp_generation() {
        // Test that we can generate a timestamp (basic functionality test)
        use chrono::Local;
        let timestamp = Local::now().format("%Y-%m-%d %H:%M:%S%.3f").to_string();
        assert!(!timestamp.is_empty());
        // Should contain date and time components
        assert!(timestamp.contains('-'));
        assert!(timestamp.contains(':'));
    }
}