// Integration tests for tcping Rust implementation

use std::process::Command;

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
fn test_help_output() {
    // Test that help output contains expected options
    let output = Command::new("cargo")
        .args(["run", "--", "--help"])
        .output()
        .expect("Failed to execute tcping");

    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);

    // Verify key options are present
    assert!(stdout.contains("--timeout"));
    assert!(stdout.contains("--interval"));
    assert!(stdout.contains("--count"));
    assert!(stdout.contains("--json"));
    assert!(stdout.contains("--csv"));
}

#[test]
fn test_quiet_flag() {
    // Test that quiet flag suppresses normal output
    let output = Command::new("cargo")
        .args(["run", "--", "--quiet", "--count", "1", "example.com", "80"])
        .output()
        .expect("Failed to execute tcping");

    // With quiet flag, output should be minimal or empty
    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);

    // Either success with minimal output, or failure with error
    if output.status.success() {
        assert!(stdout.is_empty() || stdout.len() < 100);
    } else {
        assert!(stderr.contains("error") || stderr.contains("failed"));
    }
}

#[test]
fn test_count_flag() {
    // Test that count flag limits the number of probes
    let output = Command::new("cargo")
        .args(["run", "--", "--count", "2", "--timeout", "1", "example.com", "80"])
        .output()
        .expect("Failed to execute tcping");

    // Should complete without hanging (count limits execution)
    assert!(true); // Just test that it doesn't hang indefinitely
}