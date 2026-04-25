# TCPing - Rust Implementation

A TCP ping tool written in Rust for measuring network connectivity and latency. This is a Rust rewrite of the original Go TCPing tool.

## Features

- **TCP Connectivity Testing**: Measure TCP connection establishment time and success rate
- **RTT Measurement**: Accurate round-trip time measurement with nanosecond precision
- **Cross-Platform**: Works on Linux, macOS, and other platforms supported by Rust
- **Multiple Output Formats**: Console output, JSON, CSV, and SQLite database support
- **IPv4/IPv6 Support**: Dual-stack networking with protocol family selection
- **Interface Binding**: Bind to specific network interfaces
- **Real-time Statistics**: Live statistics display and comprehensive session summary

## Installation

### From Source

```bash
# Clone the repository
git clone https://github.com/clumsy-coder/tcping.git
cd tcping

# Build the project
cargo build --release

# Install the binary
cargo install --path .
```

### Pre-built Packages

- **Ubuntu**: `.deb` package available in releases
- **macOS**: `.tar.gz` archive available in releases

## Usage

### Basic Usage

```bash
# Ping a remote host on port 80
tcping example.com 80

# Ping with IPv6 only
tcping -6 example.com 443

# Ping with custom interval and timeout
tcping -i 0.5 -t 2.0 example.com 22
```

### Command Line Options

```
USAGE:
    tcping [OPTIONS] <HOST> <PORT>

ARGS:
    <HOST>    Target hostname or IP address
    <PORT>    Target port number

OPTIONS:
    -4, --ipv4-only              Use IPv4 only
    -6, --ipv6-only              Use IPv6 only
    -r, --retry-resolution <N>   Retry hostname resolution after N failed probes
    -c, --count <N>              Stop after N probes
    -i, --interval <SECONDS>     Interval between probes in seconds [default: 1.0]
    -t, --timeout <SECONDS>      Timeout duration in seconds [default: 1.0]
    -I, --interface <INTERFACE>  Bind to specific network interface
    -j, --json                   Output in JSON format
        --pretty                 Pretty JSON formatting
        --no-color               Disable colored output
    -D, --timestamps             Show timestamps
        --csv <FILE>             Save output to CSV file
        --db <FILE>              Save output to SQLite database
        --show-source-address    Show source address
        --show-failures-only     Only show failed probes
    -v, --verbose...             Verbose output
    -u, --check-updates          Check for updates
    -h, --help                   Print help information
```

### Programmatic Usage

The library can also be used programmatically:

```rust
use tcping::{TcpPingBuilder, ProtocolFamily};
use std::time::Duration;

#[tokio::main]
async fn main() {
    let mut tcping = TcpPingBuilder::new("example.com".to_string(), 80)
        .protocol_family(ProtocolFamily::IPv4Only)
        .interval(Duration::from_secs(1))
        .timeout(Duration::from_secs(2))
        .max_probes(Some(10))
        .build();

    if let Err(e) = tcping.run().await {
        eprintln!("Error: {}", e);
    }
}
```

## Output Examples

### Console Output

```
2024-01-01 12:00:00.123 example.com:80: ✓ 45.23ms
2024-01-01 12:00:01.124 example.com:80: ✓ 46.12ms
2024-01-01 12:00:02.125 example.com:80: ✗ Connection timeout

============================== TCP Ping Statistics ==============================
Probes sent: 3
Successful: 2 (66.7%)
Failed: 1 (33.3% loss)
Average RTT: 45.68ms
Minimum RTT: 45.23ms
Maximum RTT: 46.12ms
Duration: 2.00s
Longest success streak: 2
Longest failure streak: 1
```

### JSON Output

```json
{
  "success": true,
  "rtt": 45.23,
  "error": null,
  "source_addr": "192.168.1.100:54321",
  "timestamp": "2024-01-01T12:00:00.123+00:00",
  "target_addr": "example.com:80"
}
```

## Building from Source

### Prerequisites

- Rust 1.70 or later
- Cargo (Rust's package manager)

### Build Steps

```bash
# Clone the repository
git clone https://github.com/clumsy-coder/tcping.git
cd tcping

# Build in debug mode
cargo build

# Build in release mode
cargo build --release

# Run tests
cargo test

# Run with example
cargo run -- example.com 80
```

### Cross-Compilation

For cross-compilation to different targets:

```bash
# Install cross-compilation toolchain
rustup target add x86_64-unknown-linux-musl
rustup target add x86_64-apple-darwin

# Build for Linux
cargo build --release --target x86_64-unknown-linux-musl

# Build for macOS
cargo build --release --target x86_64-apple-darwin
```

## Package Creation

### Ubuntu .deb Package

```bash
# Install cargo-deb if not already installed
cargo install cargo-deb

# Create .deb package
cargo deb
```

### macOS .tar.gz Archive

```bash
# Build for macOS
cargo build --release --target x86_64-apple-darwin

# Create archive
tar -czf tcping-macos.tar.gz -C target/x86_64-apple-darwin/release tcping
```

## Development

### Project Structure

```
src/
├── main.rs          # Entry point and CLI orchestration
├── lib.rs           # Library interface and builder pattern
├── cli.rs           # Command-line argument parsing
├── config.rs        # Configuration management
├── tcping.rs        # Core TCP ping logic and statistics
└── output.rs        # Output formatting and serialization
```

### Testing

Run the test suite:

```bash
cargo test
```

Run specific test categories:

```bash
# Unit tests only
cargo test --lib

# Integration tests
cargo test --test integration
```

## License

This project is licensed under the MIT License - see the LICENSE file for details.

## Contributing

Contributions are welcome! Please feel free to submit pull requests or open issues for bugs and feature requests.

## Acknowledgments

- Inspired by the original Go implementation of TCPing
- Built with the excellent Rust ecosystem and community tools