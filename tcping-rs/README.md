# tcping-rs

A TCP ping tool rewritten in Rust for macOS.

## Installation

1. Extract this archive:
   ```bash
   tar -xzf tcping-rs-0.1.0-x86_64-apple-darwin.tar.gz
   ```

2. Copy the binary to your PATH:
   ```bash
   sudo cp tcping /usr/local/bin/
   ```

## Usage

```bash
tcping google.com 80
tcping --count 5 --timeout 3 google.com:443
tcping --help
```

## Features

- TCP connection testing with RTT measurement
- IPv4 and IPv6 support
- Customizable timeout and count parameters
- Colorized output with --no-color option
- JSON and CSV output formats
- Real-time statistics display

## Building from Source

To build this package on macOS:

```bash
# Install Rust (if not already installed)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Build the project
cargo build --release

# Create package
make package-tar-gz
```