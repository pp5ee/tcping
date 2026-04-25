# TCPing - Rust Rewrite

A complete Rust rewrite of the original Go TCPing tool, providing TCP connectivity testing with RTT measurement, statistics, and cross-platform support.

## Features

- **TCP Connectivity Testing**: Test TCP connectivity to any host and port
- **RTT Measurement**: Accurate round-trip time measurement in milliseconds
- **Cross-Platform**: Supports Ubuntu (.deb) and macOS (.tar.gz) packages
- **Multiple Output Formats**: Text, JSON, CSV output options
- **Statistics**: Packet loss, min/avg/max RTT, success/failure streaks
- **IPv4/IPv6 Support**: Dual-stack networking support
- **Configurable**: Timeout, interval, count, and interface options

## Quick Start

### Ubuntu Installation
```bash
# Install from .deb package
sudo dpkg -i tcping_2.7.1_amd64.deb

# Usage
tcping google.com 80
tcping --help
```

### macOS Installation
```bash
# Extract and install
 tar -xzf tcping_2.7.1_macos_x86_64.tar.gz
 ./tcping google.com 80

# Or copy to /usr/local/bin
sudo cp tcping /usr/local/bin/
```

## Command Line Usage

```bash
# Basic usage
tcping <hostname> <port>

# Examples
tcping google.com 443
tcping 192.168.1.1 22
tcping example.com:80  # Combined format

# Common options
tcping -c 10 google.com 80      # Stop after 10 probes
tcping -i 0.5 google.com 80     # 500ms interval
tcping -t 2.0 google.com 80     # 2 second timeout
tcping -4 google.com 80         # IPv4 only
tcping -6 google.com 80         # IPv6 only
tcping -j google.com 80         # JSON output
tcping --no-color google.com 80 # Plain text output
```

## Advanced Features

### JSON Output
```bash
tcping -j google.com 80
# Output: {"timestamp": "...", "success": true, "target": "...", "rtt_ms": 45.2}
```

### CSV Output
```bash
tcping --csv output.csv google.com 80
```

### Interface Binding
```bash
tcping -I eth0 google.com 80      # Bind to specific interface
tcping -I 192.168.1.100 google.com 80  # Bind to specific IP
```

## Package Information

### Ubuntu Package (.deb)
- **Architecture**: amd64 (x86_64)
- **Dependencies**: Standard C library, no additional runtime dependencies
- **Installation Path**: `/usr/bin/tcping`
- **Package Manager**: Compatible with APT/dpkg

### macOS Package (.tar.gz)
- **Architectures**: x86_64 (Intel), aarch64 (Apple Silicon)
- **Dependencies**: macOS 10.15+ (Catalina)
- **Installation**: Manual extraction or copy to `/usr/local/bin/`

## Building from Source

### Prerequisites
- Rust 1.70+ toolchain
- Cargo package manager

### Build Commands
```bash
# Clone the repository
git clone <repository-url>
cd tcping

# Build for current platform
cargo build --release

# Cross-compile for Ubuntu
cargo build --release --target x86_64-unknown-linux-gnu

# Cross-compile for macOS
cargo build --release --target x86_64-apple-darwin
cargo build --release --target aarch64-apple-darwin
```

## Comparison with Go Version

### Improvements in Rust Version
- **Memory Safety**: Rust's ownership model prevents memory errors
- **Performance**: Optimized async networking with Tokio
- **Package Size**: Smaller binaries due to static linking options
- **Cross-Platform**: Simplified cross-compilation setup
- **Modern CLI**: Improved argument parsing with clap

### Preserved Features
- All core TCP probing functionality
- RTT measurement accuracy
- Statistics and output formatting
- Command-line interface compatibility
- Cross-platform support

## Release Packages Structure

### Ubuntu Release
```
tcping_2.7.1_amd64.deb
├── /usr/bin/tcping (executable)
└── DEBIAN/control (package metadata)
```

### macOS Release
```
tcping_2.7.1_macos_x86_64.tar.gz
├── tcping (executable)
└── README.txt (usage instructions)
```

## Testing

### Basic Functionality Test
```bash
# Test local connectivity
tcping localhost 22

# Test remote connectivity
tcping google.com 80

# Test with statistics
tcping -c 5 google.com 443
```

### Package Installation Test
```bash
# Ubuntu
sudo dpkg -i tcping_2.7.1_amd64.deb
tcping --version

# macOS
tar -xzf tcping_2.7.1_macos_x86_64.tar.gz
./tcping --version
```

## License

This project is licensed under the same terms as the original Go TCPing tool.

## Contributing

Contributions are welcome! Please see the CONTRIBUTING.md file for guidelines.

## Support

For issues and feature requests, please open an issue on the project repository.