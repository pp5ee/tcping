#[cfg(test)]
mod tests {
    use crate::*;
    use std::time::Duration;

    #[test]
    fn test_config_creation() {
        let config = Config {
            hostname: "127.0.0.1".to_string(),
            port: 80,
            protocol_family: ProtocolFamily::Any,
            retry_resolution: 0,
            max_probes: Some(10),
            interval: Duration::from_secs(1),
            timeout: Duration::from_secs(5),
            interface: None,
            output: OutputConfig {
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

        assert_eq!(config.port, 80);
        assert_eq!(config.max_probes, Some(10));
        assert_eq!(config.interval, Duration::from_secs(1));
        assert_eq!(config.timeout, Duration::from_secs(5));
    }

    #[test]
    fn test_output_config_creation() {
        let config = OutputConfig {
            json: false,
            pretty: false,
            color: true,
            timestamps: false,
            csv_file: None,
            db_file: None,
            show_source_address: false,
            show_failures_only: false,
        };

        assert!(!config.json);
        assert!(!config.pretty);
        assert!(!config.timestamps);
        assert!(config.color);
        assert!(!config.show_failures_only);
        assert!(!config.show_source_address);
    }

    #[test]
    fn test_cli_validation() {
        let cli = Cli {
            host: "google.com".to_string(),
            port: 80,
            ipv4_only: false,
            ipv6_only: false,
            retry_resolution: 0,
            count: Some(5),
            interval: 1.0,
            timeout: 3.0,
            interface: None,
            json: false,
            pretty: false,
            no_color: false,
            timestamps: false,
            csv: None,
            db: None,
            show_source_address: false,
            show_failures_only: false,
            verbose: 0,
            check_updates: false,
        };

        assert!(cli.validate().is_ok());
    }

    #[test]
    fn test_cli_invalid_combination() {
        let cli = Cli {
            host: "google.com".to_string(),
            port: 80,
            ipv4_only: true,
            ipv6_only: true,  // This should cause validation to fail
            retry_resolution: 0,
            count: Some(5),
            interval: 1.0,
            timeout: 3.0,
            interface: None,
            json: false,
            pretty: false,
            no_color: false,
            timestamps: false,
            csv: None,
            db: None,
            show_source_address: false,
            show_failures_only: false,
            verbose: 0,
            check_updates: false,
        };

        assert!(cli.validate().is_err());
    }
}