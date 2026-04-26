#[cfg(test)]
mod tests {
    use crate::config::{Config, OutputConfig, ProtocolFamily};
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
}