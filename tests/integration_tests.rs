// Integration tests for TCPing functionality

use std::process::Command;
use std::time::Duration;

#[test]
fn test_basic_help_output() {
    let output = Command::new("target/x86_64-unknown-linux-gnu/release/tcping")
        .arg("--help")
        .output()
        .expect("Failed to execute tcping");

    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("TCP ping utility"));
    assert!(stdout.contains("Usage: tcping"));
}

#[test]
fn test_version_output() {
    let output = Command::new("target/x86_64-unknown-linux-gnu/release/tcping")
        .arg("--version")
        .output()
        .expect("Failed to execute tcping");

    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("tcping"));
    assert!(stdout.contains("2.7.1"));
}

#[test]
fn test_invalid_hostname() {
    let output = Command::new("target/x86_64-unknown-linux-gnu/release/tcping")
        .arg("invalid-hostname-that-should-not-exist.local")
        .arg("80")
        .arg("--count")
        .arg("1")
        .output()
        .expect("Failed to execute tcping");

    // Should fail with appropriate error
    assert!(!output.status.success());
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("error") || stderr.contains("failed"),
            "Expected error message for invalid hostname");
}

#[test]
fn test_invalid_port() {
    let output = Command::new("target/x86_64-unknown-linux-gnu/release/tcping")
        .arg("localhost")
        .arg("99999") // Invalid port number
        .arg("--count")
        .arg("1")
        .output()
        .expect("Failed to execute tcping");

    // Should fail with appropriate error
    assert!(!output.status.success());
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("error") || stderr.contains("invalid"),
            "Expected error message for invalid port");
}

#[test]
fn test_no_color_option() {
    let output = Command::new("target/x86_64-unknown-linux-gnu/release/tcping")
        .arg("--help")
        .arg("--no-color")
        .output()
        .expect("Failed to execute tcping");

    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("TCP ping utility"));
    // No-color should not affect help output significantly
}

#[test]
fn test_json_output_format() {
    let output = Command::new("target/x86_64-unknown-linux-gnu/release/tcping")
        .arg("localhost")
        .arg("80")
        .arg("--count")
        .arg("1")
        .arg("--json")
        .output()
        .expect("Failed to execute tcping");

    // The command may succeed or fail depending on whether localhost:80 is accessible
    // But it should output JSON format if it does run
    let stdout = String::from_utf8_lossy(&output.stdout);
    if output.status.success() {
        // If successful, should contain JSON
        assert!(stdout.contains("{") && stdout.contains("}"),
                "Expected JSON output format");
    }
}

#[test]
fn test_timeout_option() {
    let output = Command::new("target/x86_64-unknown-linux-gnu/release/tcping")
        .arg("8.8.8.8") // Google DNS - should be reachable
        .arg("53") // DNS port - should be open
        .arg("--timeout")
        .arg("2") // 2 second timeout
        .arg("--count")
        .arg("1")
        .output()
        .expect("Failed to execute tcping");

    // This test should generally succeed if network is available
    if output.status.success() {
        let stdout = String::from_utf8_lossy(&output.stdout);
        assert!(stdout.contains("8.8.8.8") || stdout.contains("success") || stdout.contains("connected"),
                "Expected successful connection output");
    }
}

#[test]
fn test_interval_option() {
    let output = Command::new("target/x86_64-unknown-linux-gnu/release/tcping")
        .arg("--help")
        .arg("--interval")
        .arg("2")
        .output()
        .expect("Failed to execute tcping");

    // Should still show help despite interval option
    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("TCP ping utility"));
}

#[test]
fn test_verbose_option() {
    let output = Command::new("target/x86_64-unknown-linux-gnu/release/tcping")
        .arg("--help")
        .arg("--verbose")
        .output()
        .expect("Failed to execute tcping");

    // Should still show help despite verbose option
    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("TCP ping utility"));
}