use assert_cmd::Command;
use predicates::prelude::*;
use std::process::Command as StdCommand;

/// Test basic CLI compatibility with the original Go tcping tool
#[test]
fn test_cli_help_output() {
    let mut cmd = Command::cargo_bin("tcping").unwrap();
    cmd.arg("--help");
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("TCP ping utility"))
        .stdout(predicate::str::contains("Usage: tcping"))
        .stdout(predicate::str::contains("<HOST>"))
        .stdout(predicate::str::contains("<PORT>"));
}

#[test]
fn test_cli_version() {
    let mut cmd = Command::cargo_bin("tcping").unwrap();
    cmd.arg("--version");
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("tcping 2.7.1"));
}

#[test]
fn test_required_args() {
    let mut cmd = Command::cargo_bin("tcping").unwrap();
    cmd.assert().failure(); // Should fail without required args

    let mut cmd = Command::cargo_bin("tcping").unwrap();
    cmd.arg("google.com");
    cmd.assert().failure(); // Should fail without port

    let mut cmd = Command::cargo_bin("tcping").unwrap();
    cmd.arg("google.com").arg("80");
    cmd.assert().success(); // Should work with both args
}

#[test]
fn test_ipv4_ipv6_flags() {
    let mut cmd = Command::cargo_bin("tcping").unwrap();
    cmd.arg("google.com").arg("80").arg("-4");
    cmd.assert().success();

    let mut cmd = Command::cargo_bin("tcping").unwrap();
    cmd.arg("google.com").arg("80").arg("-6");
    cmd.assert().success();

    // Should fail when both -4 and -6 are specified
    let mut cmd = Command::cargo_bin("tcping").unwrap();
    cmd.arg("google.com").arg("80").arg("-4").arg("-6");
    cmd.assert().failure();
}

#[test]
fn test_count_flag() {
    let mut cmd = Command::cargo_bin("tcping").unwrap();
    cmd.arg("127.0.0.1").arg("80").arg("-c").arg("3");
    cmd.assert().success();
}

#[test]
fn test_timeout_interval() {
    let mut cmd = Command::cargo_bin("tcping").unwrap();
    cmd.arg("127.0.0.1").arg("80").arg("-t").arg("2");
    cmd.assert().success();

    let mut cmd = Command::cargo_bin("tcping").unwrap();
    cmd.arg("127.0.0.1").arg("80").arg("-i").arg("0.5");
    cmd.assert().success();
}

#[test]
fn test_output_formats() {
    // Test JSON output
    let mut cmd = Command::cargo_bin("tcping").unwrap();
    cmd.arg("127.0.0.1").arg("80").arg("--json").arg("-c").arg("1");
    cmd.assert().success();

    // Test CSV output (should create file)
    let mut cmd = Command::cargo_bin("tcping").unwrap();
    cmd.arg("127.0.0.1").arg("80").arg("--csv").arg("test.csv").arg("-c").arg("1");
    cmd.assert().success();

    // Clean up
    let _ = std::fs::remove_file("test.csv");
}

#[test]
fn test_verbose_flag() {
    let mut cmd = Command::cargo_bin("tcping").unwrap();
    cmd.arg("127.0.0.1").arg("80").arg("-v").arg("-c").arg("1");
    cmd.assert().success();

    let mut cmd = Command::cargo_bin("tcping").unwrap();
    cmd.arg("127.0.0.1").arg("80").arg("-vv").arg("-c").arg("1");
    cmd.assert().success();
}