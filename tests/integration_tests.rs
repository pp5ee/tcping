use std::process::Command;
use assert_cmd::prelude::*;
use predicates::prelude::*;

#[test]
fn test_help_output() {
    let mut cmd = Command::cargo_bin("tcping").unwrap();
    cmd.arg("--help");
    cmd.assert().success().stdout(predicate::str::contains("TCP ping utility"));
}

#[test]
fn test_version_output() {
    let mut cmd = Command::cargo_bin("tcping").unwrap();
    cmd.arg("--version");
    cmd.assert().success().stdout(predicate::str::contains("2.7.1"));
}

#[test]
fn test_missing_arguments() {
    let mut cmd = Command::cargo_bin("tcping").unwrap();
    cmd.assert().failure().stderr(predicate::str::contains("HOST"));
}

#[test]
fn test_invalid_hostname() {
    let mut cmd = Command::cargo_bin("tcping").unwrap();
    cmd.args(["invalid-hostname-that-does-not-exist.local", "80"]);
    cmd.assert().failure();
}

#[test]
fn test_invalid_port() {
    let mut cmd = Command::cargo_bin("tcping").unwrap();
    cmd.args(["google.com", "99999"]);
    cmd.assert().failure();
}