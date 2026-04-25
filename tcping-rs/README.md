# tcping-rs

A TCP ping tool rewritten in Rust from the original Go implementation.

## Features

- TCP connection probing with RTT measurement
- IPv4 and IPv6 support
- Configurable timeout and interval settings
- Multiple output formats (JSON, plain text, colored)
- Cross-platform compatibility (Linux, macOS)

## Usage

```bash
tcping <hostname> <port>
tcping <hostname:port>
```

## Building

```bash
cargo build --release
```

## Installation

Ubuntu (.deb):
```bash
sudo dpkg -i tcping-rs_0.1.0_amd64.deb
```

macOS (.tar.gz):
```bash
tar -xzf tcping-rs-0.1.0-x86_64-apple-darwin.tar.gz
sudo cp tcping /usr/local/bin/
```