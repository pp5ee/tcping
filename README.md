# TCPing - Rust Implementation

A complete Rust rewrite of the TCPing tool with 100% CLI compatibility to the original Go version. This implementation provides comprehensive TCP connection testing with advanced statistics tracking.

## Features

- **100% CLI Compatible** - Identical command-line interface to the original Go version
- **Comprehensive Statistics** - Uptime/downtime tracking, RTT min/avg/max, hostname change tracking
- **Multiple Output Formats** - Human-readable, JSON, CSV, and SQLite database output
- **Real-time Features** - Press Enter for real-time statistics, Ctrl+C for graceful shutdown
- **Cross-Platform Support** - Built with cross-compilation for Ubuntu Linux
- **Advanced Networking** - IPv4/IPv6 support, interface binding, hostname resolution retry

## Installation

### Ubuntu Package Installation

Download the latest release from GitHub and install:

```bash
# Download and extract the package
wget https://github.com/[username]/tcping/releases/latest/download/tcping_2.7.1_x86_64.tar.gz
tar -xzf tcping_2.7.1_x86_64.tar.gz

# Install system-wide
cd tcping_2.7.1_x86_64
sudo ./install.sh
```

### Building from Source

```bash
# Install Rust toolchain
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Clone and build
git clone https://github.com/[username]/tcping.git
cd tcping
cargo build --release

# Create Ubuntu package
./build-ubuntu-tar.sh
```

## Usage

Basic TCP connectivity testing:
```bash
tcping google.com 80
tcping 8.8.8.8 53
```

Advanced usage with statistics:
```bash
# Continuous monitoring with interval
tcping --interval 2 --count 10 example.com 443

# JSON output for automation
tcping --json --pretty example.com 80

# CSV output for data analysis
tcping --csv results.csv example.com 22

# Show only failures
tcping --show-failures-only --timeout 1 example.com 8080

# Real-time statistics (press Enter during execution)
tcping --count 0 example.com 443
```

## CLI Reference

Key command-line options:

- `-c, --count <COUNT>` - Stop after N probes (0 for infinite)
- `-i, --interval <INTERVAL>` - Interval between probes in seconds (default: 1)
- `-t, --timeout <TIMEOUT>` - Timeout duration in seconds (default: 5)
- `-4, --ipv4` - Use IPv4 only
- `-6, --ipv6` - Use IPv6 only
- `-j, --json` - Output in JSON format
- `--pretty` - Pretty JSON formatting
- `--csv <CSV>` - Save output to CSV file
- `--db <DB>` - Save output to SQLite database
- `--show-source-address` - Show source address
- `--show-failures-only` - Only show failed probes
- `-v, --verbose` - Verbose output
- `-u, --check-updates` - Check for updates

## Release Automation

This project includes GitHub Actions automation for:
- Automated Ubuntu package builds on tag releases
- Cross-compilation for x86_64-unknown-linux-gnu target
- Tar package creation with validation
- Release asset upload to GitHub

## License

MIT OR Apache-2.0 - See LICENSE file for details.