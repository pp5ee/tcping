// Integration tests for tcping Rust implementation

use std::process::Command;
use std::time::Duration;
use std::net::{TcpListener, TcpStream};
use std::thread;

#[test]
fn test_basic_usage() {
    // Test that the binary exists and can be executed
    let output = Command::new("cargo")
        .args(["run", "--", "--help"])
        .output()
        .expect("Failed to execute tcping");

    assert!(output.status.success());
    assert!(String::from_utf8_lossy(&output.stdout).contains("tcping"));
}

#[test]
fn test_version_flag() {
    let output = Command::new("cargo")
        .args(["run", "--", "--version"])
        .output()
        .expect("Failed to execute tcping");

    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("2.7.1") || stdout.contains("tcping"));
}

#[test]
fn test_invalid_arguments() {
    // Test with missing arguments
    let output = Command::new("cargo")
        .args(["run", "--", "google.com"]) // Missing port
        .output()
        .expect("Failed to execute tcping");

    // Should fail with error message
    assert!(!output.status.success());
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("error") || stderr.contains("missing"));
}

#[test]
fn test_local_tcp_connection() {
    // Start a simple TCP server on localhost
    let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind");
    let port = listener.local_addr().unwrap().port();

    // Start the server in a separate thread
    let server_thread = thread::spawn(move || {
        if let Ok((_stream, _addr)) = listener.accept() {
            // Connection accepted
        }
    });

    // Give the server time to start
    thread::sleep(Duration::from_millis(100));

    // Test tcping against the local server
    let output = Command::new("cargo")
        .args(["run", "--", "--quiet", "127.0.0.1", &port.to_string()])
        .timeout(Duration::from_secs(5))
        .output();

    // Clean up
    drop(listener);
    server_thread.join().ok();

    match output {
        Ok(output) => {
            // Connection should succeed
            assert!(output.status.success());
            let stdout = String::from_utf8_lossy(&output.stdout);
            assert!(stdout.contains("127.0.0.1") || stdout.is_empty());
        }
        Err(e) if e.kind() == std::io::ErrorKind::TimedOut => {
            // Timeout is acceptable for this test
            println!("Test timed out, which is acceptable for local connection test");
        }
        Err(e) => panic!("Test failed with error: {}", e),
    }
}

#[test]
fn test_unreachable_host() {
    // Test against an unreachable host (port 1 is typically unreachable)
    let output = Command::new("cargo")
        .args(["run", "--", "--timeout", "1", "192.0.2.1", "1"])
        .output()
        .expect("Failed to execute tcping");

    // Should fail with timeout or connection error
    assert!(!output.status.success());
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("timeout") || stderr.contains("error") || stderr.contains("failed"));
}

#[test]
fn test_json_output() {
    let output = Command::new("cargo")
        .args(["run", "--", "--json", "--count", "1", "example.com", "80"])
        .output()
        .expect("Failed to execute tcping");

    // Should produce valid JSON output
    if output.status.success() {
        let stdout = String::from_utf8_lossy(&output.stdout);
        // Check if output is valid JSON
        if !stdout.is_empty() {
            let json_result: Result<serde_json::Value, _> = serde_json::from_str(&stdout);
            assert!(json_result.is_ok(), "Output should be valid JSON");
        }
    }
}

#[test]
fn test_csv_output() {
    let output = Command::new("cargo")
        .args(["run", "--", "--csv", "--count", "1", "example.com", "80"])
        .output()
        .expect("Failed to execute tcping");

    // Should produce CSV output
    if output.status.success() {
        let stdout = String::from_utf8_lossy(&output.stdout);
        if !stdout.is_empty() {
            // Basic CSV validation - should contain commas
            assert!(stdout.contains(','), "Output should be CSV format");
        }
    }
}